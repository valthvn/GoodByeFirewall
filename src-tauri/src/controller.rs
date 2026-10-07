use crate::{
    config::{build_arguments, BypassConfig},
    engine::{Engine, LogSink},
    paths,
    service::{self, WindowsServices},
};
use std::os::windows::process::CommandExt;
use std::{
    path::PathBuf,
    process::Command,
    sync::{Mutex, MutexGuard},
    time::Duration,
};

#[derive(Default)]
pub struct Runtime {
    engine: Option<Engine>,
    published: bool,
    shutdown: bool,
}
#[derive(Default)]
pub struct Controller {
    runtime: Mutex<Runtime>,
}

impl Controller {
    pub fn lock(&self) -> Result<MutexGuard<'_, Runtime>, String> {
        self.runtime
            .lock()
            .map_err(|_| "État du moteur indisponible ; redémarrez l'application.".into())
    }
}

pub struct Installation {
    pub host: PathBuf,
    pub resources: PathBuf,
}
impl Installation {
    pub fn owned_paths(&self) -> Vec<PathBuf> {
        let root = if cfg!(debug_assertions) {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../BinTools/x86_64")
        } else {
            self.resources.join("BinTools/x86_64")
        };
        vec![self.host.clone(), root.join("goodbyefirewall-daemon.exe")]
    }
}

impl Runtime {
    pub fn ensure_open(&self) -> Result<(), String> {
        if self.shutdown {
            Err("L'application est en cours de fermeture.".into())
        } else {
            Ok(())
        }
    }
    pub fn status(&mut self, installation: &Installation) -> Result<bool, String> {
        if let Some(engine) = self.engine.as_mut() {
            if engine.alive()? {
                return Ok(true);
            }
            self.engine = None;
        }
        service::status(&WindowsServices, &installation.owned_paths())
    }
    pub fn publish_changed(&mut self, running: bool) -> bool {
        let changed = self.published != running;
        self.published = running;
        changed
    }
    fn stop_child(&mut self) -> Result<(), String> {
        if let Some(engine) = self.engine.as_mut() {
            engine.stop()?;
        }
        self.engine = None;
        Ok(())
    }
    pub fn start(
        &mut self,
        installation: &Installation,
        cfg: &BypassConfig,
        log: LogSink,
    ) -> Result<(), String> {
        self.ensure_open()?;
        // Validate and verify before interrupting an existing working session.
        let args = build_arguments(cfg)?;
        let exe = paths::engine_path(&installation.resources)?;
        if !cfg!(debug_assertions) {
            paths::require_protected(&installation.host)?;
            paths::require_protected(
                installation
                    .host
                    .parent()
                    .ok_or("Installation introuvable.")?,
            )?;
            paths::require_protected(&installation.resources)?;
            paths::require_protected(&installation.resources.join("BinTools"))?;
        }
        self.stop(installation)?;
        if cfg.is_service_mode {
            service::start(&installation.host, &args, &installation.owned_paths())?;
        } else {
            let mut command = Command::new(&exe);
            command
                .args(&args)
                .current_dir(exe.parent().unwrap())
                .creation_flags(0x08000000);
            let mut engine = Engine::spawn(&mut command, log)?;
            engine.observe_filter(&exe.parent().unwrap().join("WinDivert.dll"))?;
            engine.wait_ready(Duration::from_secs(10))?;
            self.engine = Some(engine);
        }
        Ok(())
    }
    pub fn stop(&mut self, installation: &Installation) -> Result<(), String> {
        self.stop_child()?;
        service::stop_owned(&WindowsServices, service::NAME, &installation.owned_paths())
    }
    pub fn uninstall(&mut self, installation: &Installation) -> Result<(), String> {
        self.ensure_open()?;
        self.stop_child()?;
        service::uninstall_owned(&WindowsServices, service::NAME, &installation.owned_paths())
    }
    pub fn migrate(&mut self, installation: &Installation) -> Result<(), String> {
        self.ensure_open()?;
        // Explicitly requested by the migration command, never by ordinary start/stop.
        service::uninstall_owned(
            &WindowsServices,
            service::LEGACY_NAME,
            &installation.owned_paths(),
        )
    }
    pub fn quit(&mut self) -> Result<(), String> {
        self.stop_child()?;
        self.shutdown = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Barrier,
    };
    #[test]
    fn competing_windows_serialize_lifecycle_operations() {
        let controller = Arc::new(Controller::default());
        let barrier = Arc::new(Barrier::new(8));
        let active = Arc::new(AtomicUsize::new(0));
        let max = Arc::new(AtomicUsize::new(0));
        let tasks: Vec<_> = (0..8)
            .map(|_| {
                let (controller, barrier, active, max) = (
                    controller.clone(),
                    barrier.clone(),
                    active.clone(),
                    max.clone(),
                );
                std::thread::spawn(move || {
                    barrier.wait();
                    let _operation = controller.lock().unwrap();
                    let n = active.fetch_add(1, Ordering::SeqCst) + 1;
                    max.fetch_max(n, Ordering::SeqCst);
                    std::thread::sleep(Duration::from_millis(5));
                    active.fetch_sub(1, Ordering::SeqCst);
                })
            })
            .collect();
        for task in tasks {
            task.join().unwrap();
        }
        assert_eq!(max.load(Ordering::SeqCst), 1);
    }
    #[test]
    fn shutdown_rejects_new_start_and_status_events_only_track_transitions() {
        let mut runtime = Runtime::default();
        assert!(runtime.publish_changed(true));
        assert!(!runtime.publish_changed(true));
        assert!(runtime.publish_changed(false));
        runtime.quit().unwrap();
        assert!(runtime.ensure_open().is_err());
    }
    #[test]
    fn quit_has_no_service_dependency_and_preserves_independent_service() {
        // quit deliberately takes no service backend or installation: it owns
        // only a direct child and cannot issue stop/delete against a service.
        let mut runtime = Runtime::default();
        runtime.quit().unwrap();
        assert!(runtime.shutdown);
    }
}
