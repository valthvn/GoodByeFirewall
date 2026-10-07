use commands::*;
use std::net::{TcpListener, TcpStream};
use std::os::windows::process::CommandExt;
use std::process::Command;
use tauri::tray::{MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Window};
use tauri_plugin_notification::NotificationExt;
const CREATE_NO_WINDOW: u32 = 0x08000000;
mod commands;
pub mod config;
mod controller;
mod engine;
mod paths;
mod reflect;
mod service;
mod service_host;

fn configure_permanent_admin() {
    if let Ok(exe) = std::env::current_exe() {
        if paths::require_protected(&exe).is_err()
            || exe
                .parent()
                .map(paths::require_protected)
                .transpose()
                .is_err()
        {
            return;
        }
        let exe_str = exe.to_string_lossy().to_string();

        // Remove runasadmin compatibility flag
        let _ = Command::new(
            paths::system_tool("reg.exe").expect("Windows system directory unavailable"),
        )
        .args([
            "delete",
            "HKCU\\Software\\Microsoft\\Windows NT\\CurrentVersion\\AppCompatFlags\\Layers",
            "/v",
            &exe_str,
            "/f",
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

        // Never overwrite an unrelated task with the same name.
        let ps_cmd = format!(
            "$ErrorActionPreference = 'Stop'; $exe = '{}'; \
             $existing = Get-ScheduledTask -TaskName 'GoodByeFirewall' -ErrorAction SilentlyContinue; \
             if ($existing -and (@($existing.Actions).Count -ne 1 -or $existing.Actions[0].Execute -ne $exe -or $existing.Actions[0].Arguments -ne '--no-task-elevate')) {{ throw 'Foreign scheduled task' }}; \
             $action = New-ScheduledTaskAction -Execute $exe -Argument '--no-task-elevate'; \
             $principal = New-ScheduledTaskPrincipal -UserId ([System.Security.Principal.WindowsIdentity]::GetCurrent().Name) -LogonType Interactive -RunLevel Highest; \
             $settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit (New-TimeSpan -Days 0); \
             Register-ScheduledTask -TaskName 'GoodByeFirewall' -Action $action -Principal $principal -Settings $settings -Force",
            exe_str.replace('\'', "''")
        );
        if let Ok(tool) = paths::system_tool("powershell.exe") {
            match Command::new(tool)
                .args([
                    "-NoProfile",
                    "-NonInteractive",
                    "-WindowStyle",
                    "Hidden",
                    "-Command",
                    &ps_cmd,
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .output()
            {
                Ok(output) if output.status.success() => {}
                Ok(output) => log::warn!(
                    "Configuration administrateur : {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                Err(err) => log::warn!("Configuration administrateur : {err}"),
            }
        }
    }
}

fn try_elevate_from_task() {
    if check_is_admin() || std::env::args().any(|a| a == "--no-task-elevate") {
        return;
    }
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let Some(parent) = exe.parent() else {
        return;
    };
    if paths::require_protected(&exe).is_err() || paths::require_protected(parent).is_err() {
        return;
    }
    let Ok(tool) = paths::system_tool("powershell.exe") else {
        return;
    };
    let script = format!(
        "$ErrorActionPreference = 'Stop'; $task = Get-ScheduledTask -TaskName 'GoodByeFirewall'; \
         if (@($task.Actions).Count -ne 1 -or $task.Actions[0].Execute -ne '{}' -or $task.Actions[0].Arguments -ne '--no-task-elevate') {{ exit 1 }}; \
         Start-ScheduledTask -InputObject $task",
        exe.to_string_lossy().replace('\'', "''")
    );
    if Command::new(tool)
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
            &script,
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
    {
        std::process::exit(0);
    }
}

#[tauri::command]
fn check_is_admin() -> bool {
    unsafe { windows_sys::Win32::UI::Shell::IsUserAnAdmin() != 0 }
}

#[tauri::command]
fn request_admin_elevation() -> CommandResult {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::ffi::OsStrExt;
        if let Ok(exe) = std::env::current_exe() {
            let op: Vec<u16> = std::ffi::OsStr::new("runas")
                .encode_wide()
                .chain(Some(0))
                .collect();
            let file: Vec<u16> = exe.as_os_str().encode_wide().chain(Some(0)).collect();
            let params: Vec<u16> = std::ffi::OsStr::new("--no-task-elevate --after-elevation")
                .encode_wide()
                .chain(Some(0))
                .collect();

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
    let _ = app.notification().builder().title(title).body(body).show();
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if std::env::args().nth(1).as_deref() == Some("--service") {
        if let Err(err) = service_host::dispatch() {
            eprintln!("{err}");
        }
        return;
    }
    // Single instance check
    // Let the unelevated parent release its listener before checking instances.
    if std::env::args()
        .any(|arg| arg == "--after-elevation" || (arg == "--no-task-elevate" && check_is_admin()))
    {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while TcpStream::connect(("127.0.0.1", 38472)).is_ok()
            && std::time::Instant::now() < deadline
        {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    // Reserve the instance before creating windows, so concurrent launches cannot race.
    let listener = match TcpListener::bind(("127.0.0.1", 38472)) {
        Ok(listener) => listener,
        Err(err) => {
            let Ok(mut stream) = TcpStream::connect(("127.0.0.1", 38472)) else {
                eprintln!("Instance listener unavailable: {err}");
                return;
            };
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
    };

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
            migrate_legacy_service,
            request_admin_elevation,
            minimize_window,
            close_window,
            open_main_window,
            quit_app
        ])
        .setup(move |app| {
            if check_is_admin() {
                std::thread::spawn(configure_permanent_admin);
            }

            // Single instance listener
            {
                let app_handle = app.handle().clone();
                std::thread::spawn(move || {
                    for mut s in listener.incoming().map_while(Result::ok) {
                            use std::io::Read;
                            let _ = s.set_read_timeout(Some(std::time::Duration::from_millis(500)));
                            let mut buf = [0u8; 16];
                            if let Ok(n) = s.read(&mut buf) {
                                if n > 0 && &buf[..n] == b"SHOW\n" {
                                    open_main_window(app_handle.clone());
                                }
                            }
                    }
                });
            }

            let initial_tooltip = "GoodByeFirewall • Protection Inactive 🔴";
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
            commands::start_monitor(app.handle().clone(), app.state::<AppState>().controller.clone());
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
