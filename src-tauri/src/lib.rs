use std::fs;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tauri::tray::{MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, Window};
use tauri_plugin_notification::NotificationExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;
mod service;

fn run_service_command(args: &[&str]) -> Result<String, String> {
    service::command_output(Command::new("sc.exe")
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .output())
}

fn start_windows_service(bin_path: &str) -> Result<(), String> {
    let exists = run_service_command(&["query", "GoodByeFirewall"]).is_ok();
    run_service_command(&[
        if exists { "config" } else { "create" },
        "GoodByeFirewall", "binPath=", bin_path, "start=", "auto",
    ])?;
    if !exists {
        let _ = run_service_command(&[
            "description", "GoodByeFirewall", "GoodByeFirewall Anti-Censorship Service",
        ]);
    }
    run_service_command(&["start", "GoodByeFirewall"])?;
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let output = run_service_command(&["query", "GoodByeFirewall"])?;
        if service::is_running(&output) { return Ok(()); }
        if Instant::now() >= deadline {
            return Err(format!("Le service n'a pas atteint l'état RUNNING : {}", output.trim()));
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

#[derive(Default)]
struct AppState {
    current_process: Arc<Mutex<Option<Child>>>,
    is_protection_running: Arc<Mutex<bool>>,
    is_service_mode: Arc<Mutex<bool>>,
    cached_exe_path: Arc<Mutex<Option<PathBuf>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BypassConfig {
    #[serde(default = "default_preset")]
    pub preset: String,
    #[serde(default)]
    pub ttl: Option<String>,
    #[serde(default)]
    pub dns: Option<String>,
    #[serde(rename = "customDnsIp")]
    pub custom_dns_ip: Option<String>,
    #[serde(rename = "customDnsPort")]
    pub custom_dns_port: Option<String>,
    #[serde(rename = "extraArgs")]
    pub extra_args: Option<String>,
    #[serde(default = "default_lang")]
    pub language: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(rename = "isServiceMode", default)]
    pub is_service_mode: bool,
}

fn default_preset() -> String {
    "-5".to_string()
}
fn default_lang() -> String {
    "fr".to_string()
}
fn default_theme() -> String {
    "light".to_string()
}

#[derive(Serialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
}

fn sanitize_arg(arg: &str) -> String {
    arg.chars()
        .filter(|&c| !matches!(c, '\r' | '\n' | '"' | '\'' | '\\' | '&' | '|' | ';' | '>' | '<' | '^' | '`' | '%'))
        .collect()
}

fn build_argument_list(_app: &AppHandle, cfg: &BypassConfig) -> Vec<String> {
    if cfg.preset == "custom" {
        let custom: Vec<String> = cfg
            .extra_args
            .as_deref()
            .unwrap_or("")
            .split_whitespace()
            .map(sanitize_arg)
            .filter(|s| !s.is_empty())
            .collect();
        return if !custom.is_empty() {
            custom
        } else {
            vec!["-5".to_string()]
        };
    }

    let mut args = Vec::new();
    let clean_preset = sanitize_arg(&cfg.preset);
    args.push(if !clean_preset.is_empty() {
        clean_preset
    } else {
        "-5".to_string()
    });

    args.push("--max-payload".to_string());
    args.push("1200".to_string());

    if let Some(ttl) = &cfg.ttl {
        if ttl != "none" {
            for part in ttl.split_whitespace() {
                let sanitized = sanitize_arg(part);
                if !sanitized.is_empty() {
                    args.push(sanitized);
                }
            }
        }
    }

    match cfg.dns.as_deref().unwrap_or("none") {
        "cloudflare" => {
            args.extend_from_slice(&[
                "--dns-addr".to_string(),
                "1.1.1.1".to_string(),
                "--dns-port".to_string(),
                "53".to_string(),
                "--dnsv6-addr".to_string(),
                "2606:4700:4700::1111".to_string(),
                "--dnsv6-port".to_string(),
                "53".to_string(),
            ]);
        }
        "quad9" => {
            args.extend_from_slice(&[
                "--dns-addr".to_string(),
                "9.9.9.9".to_string(),
                "--dns-port".to_string(),
                "53".to_string(),
                "--dnsv6-addr".to_string(),
                "2620:fe::fe".to_string(),
                "--dnsv6-port".to_string(),
                "53".to_string(),
            ]);
        }
        "fdn" => {
            // FDN
            args.extend_from_slice(&[
                "--dns-addr".to_string(),
                "80.67.169.12".to_string(),
                "--dns-port".to_string(),
                "53".to_string(),
                "--dnsv6-addr".to_string(),
                "2001:910:800::12".to_string(),
                "--dnsv6-port".to_string(),
                "53".to_string(),
            ]);
        }
        "adguard" => {
            // AdGuard
            args.extend_from_slice(&[
                "--dns-addr".to_string(),
                "94.140.14.14".to_string(),
                "--dns-port".to_string(),
                "53".to_string(),
                "--dnsv6-addr".to_string(),
                "2a10:50c0::ad1:ff".to_string(),
                "--dnsv6-port".to_string(),
                "53".to_string(),
            ]);
        }
        "google" => {
            args.extend_from_slice(&[
                "--dns-addr".to_string(),
                "8.8.8.8".to_string(),
                "--dns-port".to_string(),
                "53".to_string(),
                "--dnsv6-addr".to_string(),
                "2001:4860:4860::8888".to_string(),
                "--dnsv6-port".to_string(),
                "53".to_string(),
            ]);
        }
        "custom" => {
            if let Some(ip) = &cfg.custom_dns_ip {
                let clean_ip: String = ip
                    .chars()
                    .filter(|&c| c.is_ascii_hexdigit() || c == '.' || c == ':')
                    .collect();
                if !clean_ip.is_empty() {
                    args.push("--dns-addr".to_string());
                    args.push(clean_ip);
                    if let Some(port) = &cfg.custom_dns_port {
                        let clean_port: String = port.chars().filter(|c| c.is_ascii_digit()).collect();
                        if !clean_port.is_empty() {
                            args.push("--dns-port".to_string());
                            args.push(clean_port);
                        }
                    }
                }
            }
        }
        _ => {}
    }

    if let Some(extra) = &cfg.extra_args {
        for part in extra.split_whitespace() {
            let sanitized = sanitize_arg(part);
            if !sanitized.is_empty() {
                args.push(sanitized);
            }
        }
    }

    args
}


fn get_executable_path(app: &AppHandle, state: &AppState) -> Option<PathBuf> {
    if let Some(cached) = state.cached_exe_path.lock().unwrap().clone() {
        if cached.exists() {
            return Some(cached);
        }
    }

    let arch_dir = "x86_64";
    let exe_names = ["goodbyefirewall-daemon.exe", "goodbyedpi.exe"];

    for exe_name in &exe_names {
        if let Ok(res_dir) = app.path().resource_dir() {
            let candidate = res_dir.join("BinTools").join(arch_dir).join(exe_name);
            if candidate.exists() {
                *state.cached_exe_path.lock().unwrap() = Some(candidate.clone());
                return Some(candidate);
            }
        }

        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(dir) = exe_path.parent() {
                let candidate = dir.join("BinTools").join(arch_dir).join(exe_name);
                if candidate.exists() {
                    *state.cached_exe_path.lock().unwrap() = Some(candidate.clone());
                    return Some(candidate);
                }
                let candidate2 = dir.join("..").join("BinTools").join(arch_dir).join(exe_name);
                if candidate2.exists() {
                    *state.cached_exe_path.lock().unwrap() = Some(candidate2.clone());
                    return Some(candidate2);
                }
            }
        }

        let direct_candidates = [
            PathBuf::from("BinTools").join(arch_dir).join(exe_name),
            PathBuf::from("GoodByeFirewall").join("BinTools").join(arch_dir).join(exe_name),
            PathBuf::from("..").join("BinTools").join(arch_dir).join(exe_name),
            PathBuf::from("..").join("..").join("BinTools").join(arch_dir).join(exe_name),
        ];

        for candidate in &direct_candidates {
            if candidate.exists() {
                let resolved = candidate.canonicalize().ok().unwrap_or_else(|| candidate.clone());
                *state.cached_exe_path.lock().unwrap() = Some(resolved.clone());
                return Some(resolved);
            }
        }
    }

    None
}

fn get_config_path(app: &AppHandle) -> PathBuf {
    if let Ok(cfg_dir) = app.path().app_config_dir() {
        if !cfg_dir.exists() {
            let _ = fs::create_dir_all(&cfg_dir);
            if let Some(parent) = cfg_dir.parent() {
                let old_cfg = parent.join("com.goodbyefirewall.app").join("config.json");
                if old_cfg.exists() {
                    let _ = fs::copy(&old_cfg, cfg_dir.join("config.json"));
                }
            }
        }
        return cfg_dir.join("config.json");
    }
    PathBuf::from("config.json")
}

fn stop_all_goodbyefirewall() {
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "goodbyefirewall-daemon.exe"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "goodbyedpi.exe"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let _ = Command::new("sc.exe")
        .args(["stop", "GoodByeFirewall"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let _ = Command::new("sc.exe")
        .args(["stop", "GoodbyeDPI"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
}

fn configure_permanent_admin() {
    if let Ok(exe) = std::env::current_exe() {
        let exe_str = exe.to_string_lossy().to_string();

        // Remove runasadmin compatibility flag
        let _ = Command::new("reg.exe")
            .args([
                "delete",
                "HKCU\\Software\\Microsoft\\Windows NT\\CurrentVersion\\AppCompatFlags\\Layers",
                "/v",
                &exe_str,
                "/f",
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .output();

        // Scheduled task with battery support
        let ps_cmd = format!(
            "$action = New-ScheduledTaskAction -Execute '{}' -Argument '--no-task-elevate'; \
             $principal = New-ScheduledTaskPrincipal -UserId $env:USERNAME -LogonType Interactive -RunLevel Highest; \
             $settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit (New-TimeSpan -Days 0); \
             Register-ScheduledTask -TaskName 'GoodByeFirewall' -Action $action -Principal $principal -Settings $settings -Force",
            exe_str.replace('\'', "''")
        );

        let ps_res = Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-WindowStyle",
                "Hidden",
                "-Command",
                &ps_cmd,
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .output();

        if ps_res.is_err() || !ps_res.as_ref().map(|o| o.status.success()).unwrap_or(false) {
            let _ = Command::new("schtasks.exe")
                .args([
                    "/create",
                    "/tn",
                    "GoodByeFirewall",
                    "/tr",
                    &format!("\"{}\" --no-task-elevate", exe_str),
                    "/rl",
                    "HIGHEST",
                    "/sc",
                    "ONCE",
                    "/st",
                    "00:00",
                    "/f",
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .output();

            let _ = Command::new("powershell.exe")
                .args([
                    "-NoProfile",
                    "-NonInteractive",
                    "-WindowStyle",
                    "Hidden",
                    "-Command",
                    "$s = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries; Set-ScheduledTask -TaskName 'GoodByeFirewall' -Settings $s",
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .output();
        }
    }
}

fn try_elevate_from_task() -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--no-task-elevate") {
        return false;
    }

    if !check_is_admin() {
        let query = Command::new("schtasks.exe")
            .args(["/query", "/tn", "GoodByeFirewall", "/v", "/fo", "LIST"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();

        if let Ok(q) = query {
            if q.status.success() {
                let out_str = String::from_utf8_lossy(&q.stdout);
                if let Ok(current_exe) = std::env::current_exe() {
                    let current_exe_str = current_exe.to_string_lossy();
                    if out_str.to_lowercase().contains(&current_exe_str.to_lowercase()) {
                        let run_res = Command::new("schtasks.exe")
                            .args(["/run", "/tn", "GoodByeFirewall"])
                            .creation_flags(CREATE_NO_WINDOW)
                            .output();
                        if let Ok(r) = run_res {
                            if r.status.success() {
                                std::thread::sleep(std::time::Duration::from_millis(300));
                                std::process::exit(0);
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

#[tauri::command]
fn check_is_admin() -> bool {
    Command::new("net")
        .arg("session")
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

#[tauri::command]
fn check_status(state: tauri::State<AppState>) -> bool {
    let is_srv = *state.is_service_mode.lock().unwrap();
    if !is_srv {
        let mut proc_lock = state.current_process.lock().unwrap();
        if let Some(child) = proc_lock.as_mut() {
            match child.try_wait() {
                Ok(None) => return true,
                _ => {
                    *proc_lock = None;
                    return false;
                }
            }
        }
    }

    if is_srv {
        let out = Command::new("sc.exe")
            .args(["query", "GoodByeFirewall"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        if let Ok(o) = out {
            let s = String::from_utf8_lossy(&o.stdout);
            if o.status.success() && service::is_running(&s) {
                return true;
            }
        }
        let out_legacy = Command::new("sc.exe")
            .args(["query", "GoodbyeDPI"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        if let Ok(o) = out_legacy {
            let s = String::from_utf8_lossy(&o.stdout);
            return o.status.success() && service::is_running(&s);
        }
    } else {
        let out = Command::new("tasklist.exe")
            .args(["/FI", "IMAGENAME eq goodbyefirewall-daemon.exe", "/NH"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        if let Ok(o) = out {
            let s = String::from_utf8_lossy(&o.stdout).to_lowercase();
            if s.contains("goodbyefirewall-daemon.exe") {
                return true;
            }
        }
        let out_legacy = Command::new("tasklist.exe")
            .args(["/FI", "IMAGENAME eq goodbyedpi.exe", "/NH"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        if let Ok(o) = out_legacy {
            let s = String::from_utf8_lossy(&o.stdout).to_lowercase();
            return s.contains("goodbyedpi.exe");
        }
    }

    false
}

#[tauri::command]
fn load_config(app: AppHandle) -> serde_json::Value {
    let path = get_config_path(&app);
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(val) = serde_json::from_str(&content) {
                return val;
            }
        }
    }
    serde_json::json!({
        "preset": "-5",
        "ttl": "none",
        "dns": "cloudflare",
        "customDnsIp": "1.1.1.1",
        "customDnsPort": "53",
        "extraArgs": "",
        "language": "fr",
        "theme": "light",
        "isServiceMode": false
    })
}

#[tauri::command]
fn save_config(app: AppHandle, config: serde_json::Value) -> bool {
    let path = get_config_path(&app);
    if let Ok(serialized) = serde_json::to_string_pretty(&config) {
        let res = fs::write(path, serialized).is_ok();
        if let Some(theme) = config.get("theme").and_then(|t| t.as_str()) {
            let _ = app.emit("theme-changed", theme);
        }
        return res;
    }
    false
}

#[tauri::command]
async fn measure_ping(target_ip: String) -> Option<u128> {
    tauri::async_runtime::spawn_blocking(move || {
        let clean_ip = target_ip.trim();
        let ip: std::net::IpAddr = clean_ip.parse().unwrap_or_else(|_| "1.1.1.1".parse().unwrap());
        let socket_addr = std::net::SocketAddr::new(ip, 53);
        let start = Instant::now();

        if TcpStream::connect_timeout(&socket_addr, Duration::from_millis(1500)).is_ok() {
            return Some(start.elapsed().as_millis());
        }
        None
    })
    .await
    .unwrap_or(None)
}

#[tauri::command]
async fn start_bypass(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    config: BypassConfig,
) -> Result<CommandResult, String> {
    let exe_path = match get_executable_path(&app, &state) {
        Some(p) => p,
        None => {
            return Ok(CommandResult {
                success: false,
                message: "Exécutable goodbyefirewall-daemon.exe introuvable.".to_string(),
            });
        }
    };

    let args = build_argument_list(&app, &config);
    let args_string = args.join(" ");

    stop_all_goodbyefirewall();

    let is_srv = config.is_service_mode;
    *state.is_service_mode.lock().unwrap() = is_srv;

    if is_srv {
        let bin_path_val = format!("\"{}\" {}", exe_path.display(), args_string);

        let _ = Command::new("sc.exe")
            .args(["stop", "GoodbyeDPI"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        let _ = Command::new("sc.exe")
            .args(["delete", "GoodbyeDPI"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();

        if let Err(err) = start_windows_service(&bin_path_val) {
            *state.is_protection_running.lock().unwrap() = false;
            let _ = app.emit("status-changed", false);
            update_tray_ui(&app, false);
            return Ok(CommandResult {
                success: false,
                message: format!("Échec démarrage service : {}", err),
            });
        }

        *state.is_protection_running.lock().unwrap() = true;
        let _ = app.emit("status-changed", true);
        update_tray_ui(&app, true);
        let _ = app.emit(
            "log-message",
            format!("[Service] Démarré: goodbyefirewall-daemon.exe {}", args_string),
        );

        Ok(CommandResult {
            success: true,
            message: "Service Windows GoodByeFirewall démarré.".to_string(),
        })
    } else {
        let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new("."));
        let mut cmd = Command::new(&exe_path);
        cmd.args(&args)
            .current_dir(exe_dir)
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        match cmd.spawn() {
            Ok(mut child) => {
                let stdout = child.stdout.take();
                let stderr = child.stderr.take();

                let app_stdout = app.clone();
                if let Some(out) = stdout {
                    std::thread::spawn(move || {
                        let reader = BufReader::new(out);
                        for line in reader.lines().map_while(Result::ok) {
                            let _ = app_stdout.emit("log-message", line);
                        }
                    });
                }

                let app_stderr = app.clone();
                if let Some(err) = stderr {
                    std::thread::spawn(move || {
                        let reader = BufReader::new(err);
                        for line in reader.lines().map_while(Result::ok) {
                            let _ = app_stderr.emit("log-message", format!("[ERR] {}", line));
                        }
                    });
                }

                *state.current_process.lock().unwrap() = Some(child);
                *state.is_protection_running.lock().unwrap() = true;

                let _ = app.emit("status-changed", true);
                update_tray_ui(&app, true);
                let _ = app.emit(
                    "log-message",
                    format!("[Session] Lancé: goodbyefirewall-daemon.exe {}", args_string),
                );

                Ok(CommandResult {
                    success: true,
                    message: "Processus GoodByeFirewall lancé avec succès.".to_string(),
                })
            }
            Err(e) => {
                *state.is_protection_running.lock().unwrap() = false;
                let _ = app.emit("status-changed", false);
                update_tray_ui(&app, false);
                Ok(CommandResult {
                    success: false,
                    message: format!("Erreur au lancement: {}", e),
                })
            }
        }
    }
}

#[tauri::command]
fn stop_bypass(app: AppHandle, state: tauri::State<AppState>) -> CommandResult {
    let mut proc_lock = state.current_process.lock().unwrap();
    if let Some(mut child) = proc_lock.take() {
        let _ = child.kill();
        let _ = child.wait();
    }

    stop_all_goodbyefirewall();

    *state.is_protection_running.lock().unwrap() = false;
    let _ = app.emit("status-changed", false);
    let _ = app.emit("log-message", "[Système] Protection désactivée.");
    update_tray_ui(&app, false);

    CommandResult {
        success: true,
        message: "Protection arrêtée.".to_string(),
    }
}

#[tauri::command]
fn uninstall_service(app: AppHandle, state: tauri::State<AppState>) -> CommandResult {
    let _ = stop_bypass(app.clone(), state);

    let _ = Command::new("sc.exe")
        .args(["delete", "GoodByeFirewall"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let _ = Command::new("sc.exe")
        .args(["delete", "GoodbyeDPI"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let _ = Command::new("sc.exe")
        .args(["stop", "WinDivert"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let _ = Command::new("sc.exe")
        .args(["delete", "WinDivert"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let _ = app.emit("status-changed", false);
    let _ = app.emit(
        "log-message",
        "[Service] Service GoodByeFirewall et pilote WinDivert supprimés.",
    );

    CommandResult {
        success: true,
        message: "Service et pilotes WinDivert désinstallés.".to_string(),
    }
}

#[tauri::command]
fn request_admin_elevation() -> CommandResult {
    configure_permanent_admin();
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::ffi::OsStrExt;
        if let Ok(exe) = std::env::current_exe() {
            let op: Vec<u16> = std::ffi::OsStr::new("runas").encode_wide().chain(Some(0)).collect();
            let file: Vec<u16> = exe.as_os_str().encode_wide().chain(Some(0)).collect();
            let params: Vec<u16> = std::ffi::OsStr::new("--no-task-elevate").encode_wide().chain(Some(0)).collect();

            #[link(name = "shell32")]
            extern "system" {
                fn ShellExecuteW(
                    hwnd: *mut std::ffi::c_void,
                    lpOperation: *const u16,
                    lpFile: *const u16,
                    lpParameters: *const u16,
                    lpDirectory: *const u16,
                    nShowCmd: i32,
                ) -> isize;
            }

            let ret = unsafe {
                ShellExecuteW(
                    std::ptr::null_mut(),
                    op.as_ptr(),
                    file.as_ptr(),
                    params.as_ptr(),
                    std::ptr::null(),
                    1,
                )
            };

            if ret > 32 {
                std::process::exit(0);
            } else {
                return CommandResult {
                    success: false,
                    message: "Élévation annulée ou refusée par l'utilisateur.".to_string(),
                };
            }
        }
    }

    CommandResult {
        success: false,
        message: "Impossible de déterminer l'exécutable pour l'élévation.".to_string(),
    }
}


fn update_tray_ui(app: &AppHandle, is_running: bool) {
    if let Some(tray) = app.tray_by_id("main_tray") {
        let tooltip = if is_running {
            "GoodByeFirewall • Protection Active 🟢"
        } else {
            "GoodByeFirewall • Protection Inactive 🔴"
        };
        let _ = tray.set_tooltip(Some(tooltip));
    }
}

fn notify_user(app: &AppHandle, title: &str, body: &str) {
    let _ = app
        .notification()
        .builder()
        .title(title)
        .body(body)
        .show();
}

#[tauri::command]
fn minimize_window(window: Window) {
    let _ = window.minimize();
}

#[tauri::command]
fn close_window(app: AppHandle, window: Window) {
    let _ = window.hide();
    notify_user(
        &app,
        "GoodByeFirewall",
        "L'application continue de fonctionner en arrière-plan dans les icônes cachées.",
    );
}

#[tauri::command]
fn open_main_window(app: AppHandle) {
    if let Some(tray_w) = app.get_webview_window("tray") {
        let _ = tray_w.hide();
    }
    if let Some(main_w) = app.get_webview_window("main") {
        let _ = main_w.show();
        let _ = main_w.unminimize();
        let _ = main_w.set_focus();
        #[cfg(target_os = "windows")]
        {
            let _ = main_w.set_always_on_top(true);
            let _ = main_w.set_always_on_top(false);
        }
    }
}

#[tauri::command]
fn quit_app(app: AppHandle, state: tauri::State<AppState>) {
    let _ = stop_bypass(app.clone(), state);
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Single instance check
    if let Ok(mut stream) = TcpStream::connect(("127.0.0.1", 38472)) {
        use std::io::Write;
        #[cfg(target_os = "windows")]
        {
            #[link(name = "user32")]
            extern "system" {
                fn AllowSetForegroundWindow(dwProcessId: u32) -> i32;
            }
            const ASFW_ANY: u32 = 0xFFFFFFFF;
            unsafe {
                AllowSetForegroundWindow(ASFW_ANY);
            }
        }
        let _ = stream.write_all(b"SHOW\n");
        let _ = stream.flush();
        std::process::exit(0);
    }

    try_elevate_from_task();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            check_is_admin,
            check_status,
            load_config,
            save_config,
            measure_ping,
            start_bypass,
            stop_bypass,
            uninstall_service,
            request_admin_elevation,
            minimize_window,
            close_window,
            open_main_window,
            quit_app
        ])
        .setup(|app| {
            if check_is_admin() {
                configure_permanent_admin();
            }

            // Single instance listener
            if let Ok(listener) = TcpListener::bind(("127.0.0.1", 38472)) {
                let app_handle = app.handle().clone();
                std::thread::spawn(move || {
                    for stream in listener.incoming() {
                        if let Ok(mut s) = stream {
                            use std::io::Read;
                            let _ = s.set_read_timeout(Some(std::time::Duration::from_millis(500)));
                            let mut buf = [0u8; 16];
                            if let Ok(n) = s.read(&mut buf) {
                                if n > 0 && &buf[..n] == b"SHOW\n" {
                                    open_main_window(app_handle.clone());
                                }
                            }
                        }
                    }
                });
            }

            let state = app.state::<AppState>();
            *state.is_service_mode.lock().unwrap() = load_config(app.handle().clone())
                .get("isServiceMode")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            let initial_running = check_status(state);
            let initial_tooltip = if initial_running {
                "GoodByeFirewall • Protection Active 🟢"
            } else {
                "GoodByeFirewall • Protection Inactive 🔴"
            };

            let _tray = TrayIconBuilder::with_id("main_tray")
                .icon(app.default_window_icon().cloned().expect("Aucune icône par défaut trouvée"))
                .tooltip(initial_tooltip)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button_state: MouseButtonState::Up,
                        position,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(tray_w) = app.get_webview_window("tray") {
                            if tray_w.is_visible().unwrap_or(false) {
                                let _ = tray_w.hide();
                                return;
                            }

                            // Calculate position anchored above the taskbar with DPI & monitor bounds
                            let size = tray_w.outer_size().unwrap_or(tauri::PhysicalSize::new(300, 280));
                            let scale = tray_w.scale_factor().unwrap_or(1.0);
                            let w = size.width as f64;
                            let h = size.height as f64;

                            let mut target_x = position.x - (w / 2.0);
                            let target_y = (position.y - h - (12.0 * scale)).max(10.0);

                            if let Ok(Some(mon)) = tray_w.current_monitor() {
                                let mon_x = mon.position().x as f64;
                                let mon_w = mon.size().width as f64;
                                let min_x = mon_x + (10.0 * scale);
                                let max_x = mon_x + mon_w - w - (10.0 * scale);
                                target_x = target_x.clamp(min_x, max_x);
                            } else {
                                target_x = target_x.max(10.0);
                            }

                            let _ = tray_w.set_position(tauri::PhysicalPosition::new(target_x as i32, target_y as i32));
                            let _ = tray_w.show();
                            let _ = tray_w.set_focus();
                        }
                    }
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == "tray" {
                if let tauri::WindowEvent::Focused(false) = event {
                    let _ = window.hide();
                }
            } else if window.label() == "main" {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    let _ = window.hide();
                    api.prevent_close();
                    notify_user(
                        window.app_handle(),
                        "GoodByeFirewall",
                        "L'application continue de fonctionner en arrière-plan dans les icônes cachées.",
                    );
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("Erreur lors de l'exécution de GoodByeFirewall Tauri");
}
