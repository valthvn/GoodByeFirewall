use crate::{
    config::BypassConfig,
    controller::{Controller, Installation, Runtime},
    paths,
};
use serde::Serialize;
use std::{
    fs,
    net::TcpStream,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter, Manager};
use windows_sys::Win32::Storage::FileSystem::{
    MoveFileExW, MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH,
};

#[derive(Default)]
pub struct AppState {
    pub controller: Arc<Controller>,
    config_lock: Arc<Mutex<()>>,
}
#[derive(Serialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
}

fn installation(app: &AppHandle) -> Result<Installation, String> {
    Ok(Installation {
        host: std::env::current_exe().map_err(|err| err.to_string())?,
        resources: app.path().resource_dir().map_err(|err| err.to_string())?,
    })
}

fn publish(app: &AppHandle, runtime: &mut Runtime, running: bool) {
    if runtime.publish_changed(running) {
        let _ = app.emit("status-changed", running);
        crate::update_tray_ui(app, running);
    }
}

async fn operate(
    app: AppHandle,
    controller: Arc<Controller>,
    success: &'static str,
    action: impl FnOnce(&mut Runtime, &Installation) -> Result<(), String> + Send + 'static,
) -> Result<CommandResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let installation = installation(&app)?;
        let mut runtime = controller.lock()?;
        runtime.ensure_open()?;
        let result = action(&mut runtime, &installation);
        let running = runtime.status(&installation).unwrap_or(false);
        publish(&app, &mut runtime, running);
        let (success, message) = match result {
            Ok(()) => (true, success.to_owned()),
            Err(err) => (false, err),
        };
        let _ = app.emit("log-message", &message);
        Ok(CommandResult { success, message })
    })
    .await
    .map_err(|err| err.to_string())?
}

#[tauri::command]
pub async fn start_bypass(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    config: BypassConfig,
) -> Result<CommandResult, String> {
    let log_app = app.clone();
    let log = Arc::new(move |line| {
        let _ = log_app.emit("log-message", line);
    });
    operate(
        app,
        state.controller.clone(),
        "Filtre du moteur activé.",
        move |runtime, installation| runtime.start(installation, &config, log),
    )
    .await
}
#[tauri::command]
pub async fn stop_bypass(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<CommandResult, String> {
    operate(
        app,
        state.controller.clone(),
        "Protection arrêtée.",
        |runtime, installation| runtime.stop(installation),
    )
    .await
}
#[tauri::command]
pub async fn uninstall_service(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<CommandResult, String> {
    operate(
        app,
        state.controller.clone(),
        "Service GoodByeFirewall désinstallé. Le pilote partagé est conservé.",
        |runtime, installation| runtime.uninstall(installation),
    )
    .await
}
#[tauri::command]
pub async fn migrate_legacy_service(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<CommandResult, String> {
    operate(
        app,
        state.controller.clone(),
        "Ancien service de cette installation retiré. Vous pouvez activer le nouveau service.",
        |runtime, installation| runtime.migrate(installation),
    )
    .await
}
#[tauri::command]
pub async fn check_status(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    let controller = state.controller.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let installation = installation(&app)?;
        let mut runtime = controller.lock()?;
        let running = runtime.status(&installation)?;
        publish(&app, &mut runtime, running);
        Ok(running)
    })
    .await
    .map_err(|err| err.to_string())?
}

pub fn start_monitor(app: AppHandle, controller: Arc<Controller>) {
    std::thread::spawn(move || {
        let Ok(installation) = installation(&app) else {
            return;
        };
        let mut last_error = None;
        loop {
            std::thread::sleep(Duration::from_millis(500));
            let Ok(mut runtime) = controller.lock() else {
                return;
            };
            if runtime.ensure_open().is_err() {
                return;
            }
            let running = match runtime.status(&installation) {
                Ok(running) => {
                    last_error = None;
                    running
                }
                Err(err) => {
                    if last_error.as_ref() != Some(&err) {
                        let _ = app.emit("log-message", &err);
                        last_error = Some(err);
                    }
                    false
                }
            };
            publish(&app, &mut runtime, running);
        }
    });
}

#[tauri::command]
pub async fn quit_app(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<CommandResult, String> {
    let controller = state.controller.clone();
    tauri::async_runtime::spawn_blocking(move || {
        controller.lock()?.quit()?;
        app.exit(0);
        Ok(CommandResult {
            success: true,
            message: "Interface fermée.".into(),
        })
    })
    .await
    .map_err(|err| err.to_string())?
}

fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let root = app.path().app_config_dir().map_err(|err| err.to_string())?;
    fs::create_dir_all(&root).map_err(|err| err.to_string())?;
    let path = root.join("config.json");
    if !path.exists() {
        if let Some(parent) = root.parent() {
            let legacy = parent.join("com.goodbyefirewall.app/config.json");
            if legacy.is_file() {
                fs::copy(legacy, &path).map_err(|err| err.to_string())?;
            }
        }
    }
    Ok(path)
}

#[tauri::command]
pub async fn load_config(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<BypassConfig, String> {
    let lock = state.config_lock.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let _lock = lock
            .lock()
            .map_err(|_| "Verrou de configuration indisponible.")?;
        let path = config_path(&app)?;
        let mut cfg: BypassConfig = match fs::read(&path) {
            Ok(bytes) if bytes.len() <= 65536 => serde_json::from_slice(&bytes).unwrap_or_default(),
            Ok(_) => BypassConfig::default(),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => BypassConfig::default(),
            Err(err) => return Err(err.to_string()),
        };
        if cfg.dns.as_deref() == Some("yandex") {
            cfg.dns = Some("cloudflare".into());
        }
        Ok(cfg)
    })
    .await
    .map_err(|err| err.to_string())?
}

#[tauri::command]
pub async fn save_config(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    config: BypassConfig,
) -> Result<bool, String> {
    let lock = state.config_lock.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let _lock = lock
            .lock()
            .map_err(|_| "Verrou de configuration indisponible.")?;
        let path = config_path(&app)?;
        let bytes = serde_json::to_vec_pretty(&config).map_err(|err| err.to_string())?;
        if bytes.len() > 65536 {
            return Err("Configuration trop volumineuse.".into());
        }
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, bytes).map_err(|err| err.to_string())?;
        let (from, to) = (
            paths::wide(&tmp.to_string_lossy()),
            paths::wide(&path.to_string_lossy()),
        );
        if unsafe {
            MoveFileExW(
                from.as_ptr(),
                to.as_ptr(),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
            )
        } == 0
        {
            return Err(std::io::Error::last_os_error().to_string());
        }
        let _ = app.emit("theme-changed", &config.theme);
        Ok(true)
    })
    .await
    .map_err(|err| err.to_string())?
}

#[tauri::command]
pub async fn measure_ping(target_ip: String) -> Option<u128> {
    tauri::async_runtime::spawn_blocking(move || {
        let ip = target_ip.trim().parse::<std::net::IpAddr>().ok()?;
        let start = Instant::now();
        TcpStream::connect_timeout(
            &std::net::SocketAddr::new(ip, 53),
            Duration::from_millis(1500),
        )
        .ok()?;
        Some(start.elapsed().as_millis())
    })
    .await
    .unwrap_or(None)
}
