use std::{
    io::{BufRead, BufReader, Read},
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

pub const READY: &str = "Filter activated, GoodbyeDPI is now running!";
pub type LogSink = Arc<dyn Fn(String) + Send + Sync>;
struct Job(usize);
impl Drop for Job {
    fn drop(&mut self) {
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.0 as _);
        }
    }
}
impl Job {
    fn attach(child: &Child) -> Result<Self, String> {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::System::JobObjects::*;
        let handle = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        if handle.is_null() {
            return Err(std::io::Error::last_os_error().to_string());
        }
        let job = Self(handle as usize);
        let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        if unsafe {
            SetInformationJobObject(
                handle,
                JobObjectExtendedLimitInformation,
                (&limits as *const JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(),
                std::mem::size_of_val(&limits) as u32,
            )
        } == 0
            || unsafe { AssignProcessToJobObject(handle, child.as_raw_handle()) } == 0
        {
            return Err(std::io::Error::last_os_error().to_string());
        }
        Ok(job)
    }
}

pub struct Engine {
    child: Child,
    ready: Arc<AtomicBool>,
    failed: Arc<AtomicBool>,
    observer: Option<crate::reflect::Observer>,
    _job: Job,
}
impl Engine {
    pub fn spawn(command: &mut Command, log: LogSink) -> Result<Self, String> {
        let mut child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| err.to_string())?;
        let job = match Job::attach(&child) {
            Ok(job) => job,
            Err(err) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(err);
            }
        };
        let ready = Arc::new(AtomicBool::new(false));
        let failed = Arc::new(AtomicBool::new(false));
        if let Some(stdout) = child.stdout.take() {
            read_output(stdout, log.clone(), ready.clone(), failed.clone(), false);
        }
        if let Some(stderr) = child.stderr.take() {
            read_output(stderr, log, ready.clone(), failed.clone(), true);
        }
        Ok(Self {
            child,
            ready,
            failed,
            observer: None,
            _job: job,
        })
    }
    pub fn observe_filter(&mut self, dll: &std::path::Path) -> Result<(), String> {
        self.observer = Some(crate::reflect::Observer::start(
            dll,
            self.child.id(),
            self.ready.clone(),
            self.failed.clone(),
        )?);
        Ok(())
    }
    pub fn wait_ready(&mut self, timeout: Duration) -> Result<(), String> {
        let deadline = Instant::now() + timeout;
        loop {
            if !self.alive()? {
                return Err("Le moteur s'est arrêté avant l'activation du filtre.".into());
            }
            if self.failed.load(Ordering::Acquire) {
                return Err(
                    "Le moteur a signalé une erreur de filtre ; consultez la console.".into(),
                );
            }
            if self.ready.load(Ordering::Acquire) {
                return Ok(());
            }
            if Instant::now() >= deadline {
                return Err(
                    "Le moteur n'a pas confirmé l'activation du filtre dans le délai imparti."
                        .into(),
                );
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }
    pub fn alive(&mut self) -> Result<bool, String> {
        let alive = self
            .child
            .try_wait()
            .map_err(|err| err.to_string())?
            .is_none();
        Ok(alive && !self.failed.load(Ordering::Acquire))
    }
    pub fn stop(&mut self) -> Result<(), String> {
        if self
            .child
            .try_wait()
            .map_err(|err| err.to_string())?
            .is_none()
        {
            self.child.kill().map_err(|err| err.to_string())?;
        }
        self.child.wait().map_err(|err| err.to_string())?;
        Ok(())
    }
}
impl Drop for Engine {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

fn read_output(
    output: impl Read + Send + 'static,
    log: LogSink,
    ready: Arc<AtomicBool>,
    failed: Arc<AtomicBool>,
    stderr: bool,
) {
    std::thread::spawn(move || {
        for line in BufReader::new(output).lines().map_while(Result::ok) {
            if !stderr && line.trim() == READY {
                ready.store(true, Ordering::Release);
            }
            if line.contains("Error opening filter:") || line.contains("Error receiving packet!") {
                failed.store(true, Ordering::Release);
            }
            log(if stderr {
                format!("[ERR] {line}")
            } else {
                line
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    fn command(script: &str) -> Command {
        let mut command = Command::new(crate::paths::system_tool("powershell.exe").unwrap());
        command.args(["-NoProfile", "-NonInteractive", "-Command", script]);
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
        command
    }
    #[test]
    fn only_explicit_filter_activation_confirms_startup_and_exit_is_detected() {
        let mut engine = Engine::spawn(
            &mut command(&format!(
                "[Console]::WriteLine('{READY}'); Start-Sleep -Milliseconds 1500"
            )),
            Arc::new(|_| {}),
        )
        .unwrap();
        engine.wait_ready(Duration::from_secs(5)).unwrap();
        engine.child.wait().unwrap();
        assert!(!engine.alive().unwrap());
    }
    #[test]
    fn live_but_unready_process_times_out_and_only_owned_child_is_stopped() {
        let mut engine =
            Engine::spawn(&mut command("Start-Sleep -Seconds 30"), Arc::new(|_| {})).unwrap();
        assert!(engine.wait_ready(Duration::from_millis(100)).is_err());
        engine.stop().unwrap();
        assert!(!engine.alive().unwrap());
    }
    #[test]
    fn filter_failure_never_reports_ready() {
        let mut engine = Engine::spawn(
            &mut command(
                "[Console]::WriteLine('Error opening filter: 5'); Start-Sleep -Seconds 30",
            ),
            Arc::new(|_| {}),
        )
        .unwrap();
        assert!(engine.wait_ready(Duration::from_secs(5)).is_err());
    }
    #[test]
    fn closing_parent_job_terminates_child() {
        let mut child = command("Start-Sleep -Seconds 30").spawn().unwrap();
        let job = Job::attach(&child).unwrap();
        drop(job);
        let deadline = Instant::now() + Duration::from_secs(5);
        while child.try_wait().unwrap().is_none() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        let exited = child.try_wait().unwrap().is_some();
        if !exited {
            let _ = child.kill();
        }
        child.wait().unwrap();
        assert!(exited);
    }
}
