use crate::{
    config::{parse_arguments, quote_argument},
    paths::wide,
};
use std::{
    path::{Path, PathBuf},
    thread,
    time::{Duration, Instant},
};
use windows_sys::Win32::{
    Foundation::{
        ERROR_SERVICE_ALREADY_RUNNING, ERROR_SERVICE_DOES_NOT_EXIST, ERROR_SERVICE_NOT_ACTIVE,
    },
    System::Services::*,
};

pub const NAME: &str = "GoodByeFirewall";
pub const LEGACY_NAME: &str = "GoodbyeDPI";
struct Handle(SC_HANDLE);
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseServiceHandle(self.0);
        }
    }
}
fn manager(access: u32) -> Result<Handle, String> {
    let handle = unsafe { OpenSCManagerW(std::ptr::null(), std::ptr::null(), access) };
    if handle.is_null() {
        Err(std::io::Error::last_os_error().to_string())
    } else {
        Ok(Handle(handle))
    }
}
fn open(name: &str, access: u32) -> Result<Option<Handle>, String> {
    let scm = manager(SC_MANAGER_CONNECT)?;
    let name = wide(name);
    let handle = unsafe { OpenServiceW(scm.0, name.as_ptr(), access) };
    if !handle.is_null() {
        return Ok(Some(Handle(handle)));
    }
    let err = std::io::Error::last_os_error();
    if err.raw_os_error() == Some(ERROR_SERVICE_DOES_NOT_EXIST as i32) {
        Ok(None)
    } else {
        Err(err.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct Info {
    pub command: String,
    pub state: u32,
    pub exit_code: u32,
}
fn read_info(handle: &Handle) -> Result<Info, String> {
    let mut buf = vec![0usize; 8192 / std::mem::size_of::<usize>()];
    let mut needed = 0;
    let cfg = buf.as_mut_ptr().cast::<QUERY_SERVICE_CONFIGW>();
    if unsafe { QueryServiceConfigW(handle.0, cfg, 8192, &mut needed) } == 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    let command = unsafe {
        let ptr = (*cfg).lpBinaryPathName;
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        String::from_utf16_lossy(std::slice::from_raw_parts(ptr, len))
    };
    let mut status = SERVICE_STATUS::default();
    if unsafe { QueryServiceStatus(handle.0, &mut status) } == 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    Ok(Info {
        command,
        state: status.dwCurrentState,
        exit_code: status.dwWin32ExitCode,
    })
}

pub trait Backend {
    fn query(&self, name: &str) -> Result<Option<Info>, String>;
    fn stop(&self, name: &str) -> Result<(), String>;
    fn delete(&self, name: &str) -> Result<(), String>;
}
pub struct WindowsServices;
impl Backend for WindowsServices {
    fn query(&self, name: &str) -> Result<Option<Info>, String> {
        open(name, SERVICE_QUERY_CONFIG | SERVICE_QUERY_STATUS)?
            .as_ref()
            .map(read_info)
            .transpose()
    }
    fn stop(&self, name: &str) -> Result<(), String> {
        let Some(handle) = open(name, SERVICE_STOP | SERVICE_QUERY_STATUS)? else {
            return Ok(());
        };
        let mut status = SERVICE_STATUS::default();
        if unsafe { ControlService(handle.0, SERVICE_CONTROL_STOP, &mut status) } == 0 {
            let err = std::io::Error::last_os_error();
            if err.raw_os_error() != Some(ERROR_SERVICE_NOT_ACTIVE as i32) {
                return Err(err.to_string());
            }
        }
        Ok(())
    }
    fn delete(&self, name: &str) -> Result<(), String> {
        let Some(handle) = open(name, 0x10000)? else {
            return Ok(());
        };
        if unsafe { DeleteService(handle.0) } == 0 {
            return Err(std::io::Error::last_os_error().to_string());
        }
        Ok(())
    }
}

fn owns(info: &Info, expected: &[PathBuf]) -> Result<(), String> {
    let path = parse_arguments(&info.command)?
        .into_iter()
        .next()
        .ok_or("Service sans exécutable.")?;
    let actual = Path::new(&path)
        .canonicalize()
        .map_err(|err| format!("Chemin de service invérifiable : {err}"))?;
    if expected
        .iter()
        .any(|path| path.canonicalize().ok().as_ref() == Some(&actual))
    {
        Ok(())
    } else {
        Err(
            "Ce service appartient à une autre installation ; aucune modification effectuée."
                .into(),
        )
    }
}
pub fn status(backend: &impl Backend, expected: &[PathBuf]) -> Result<bool, String> {
    let Some(info) = backend.query(NAME)? else {
        return Ok(false);
    };
    owns(&info, expected)?;
    Ok(info.state == SERVICE_RUNNING)
}
fn wait_for(
    backend: &impl Backend,
    name: &str,
    expected: &[PathBuf],
    target: Option<u32>,
    timeout: Duration,
) -> Result<(), String> {
    let deadline = Instant::now() + timeout;
    loop {
        match backend.query(name)? {
            None if target.is_none() || target == Some(SERVICE_STOPPED) => return Ok(()),
            None => return Err("Le service a disparu pendant son démarrage.".into()),
            Some(info) => {
                owns(&info, expected)?;
                if Some(info.state) == target {
                    return Ok(());
                }
                if target == Some(SERVICE_RUNNING) && info.state == SERVICE_STOPPED {
                    return Err(format!(
                        "Le moteur n'a pas démarré (code {}).",
                        info.exit_code
                    ));
                }
            }
        }
        if Instant::now() >= deadline {
            return Err("Délai d'attente du service dépassé.".into());
        }
        thread::sleep(Duration::from_millis(100));
    }
}
pub fn stop_owned(backend: &impl Backend, name: &str, expected: &[PathBuf]) -> Result<(), String> {
    if let Some(info) = backend.query(name)? {
        owns(&info, expected)?;
        if info.state != SERVICE_STOPPED {
            backend.stop(name)?;
        }
        wait_for(
            backend,
            name,
            expected,
            Some(SERVICE_STOPPED),
            Duration::from_secs(15),
        )?;
    }
    Ok(())
}
pub fn uninstall_owned(
    backend: &impl Backend,
    name: &str,
    expected: &[PathBuf],
) -> Result<(), String> {
    stop_owned(backend, name, expected)?;
    if let Some(info) = backend.query(name)? {
        owns(&info, expected)?;
        backend.delete(name)?;
        wait_for(backend, name, expected, None, Duration::from_secs(5))?;
    }
    Ok(())
}
pub fn start(host: &Path, args: &[String], expected: &[PathBuf]) -> Result<(), String> {
    stop_owned(&WindowsServices, NAME, expected)?;
    let command = std::iter::once(host.to_string_lossy().into_owned())
        .chain(std::iter::once("--service".into()))
        .chain(args.iter().cloned())
        .map(|arg| quote_argument(&arg))
        .collect::<Vec<_>>()
        .join(" ");
    let command = wide(&command);
    if let Some(handle) = open(NAME, SERVICE_CHANGE_CONFIG)? {
        if unsafe {
            ChangeServiceConfigW(
                handle.0,
                SERVICE_NO_CHANGE,
                SERVICE_AUTO_START,
                SERVICE_NO_CHANGE,
                command.as_ptr(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
            )
        } == 0
        {
            return Err(std::io::Error::last_os_error().to_string());
        }
    } else {
        let scm = manager(SC_MANAGER_CREATE_SERVICE)?;
        let name = wide(NAME);
        let handle = unsafe {
            CreateServiceW(
                scm.0,
                name.as_ptr(),
                name.as_ptr(),
                SERVICE_QUERY_STATUS,
                SERVICE_WIN32_OWN_PROCESS,
                SERVICE_AUTO_START,
                SERVICE_ERROR_NORMAL,
                command.as_ptr(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
            )
        };
        if handle.is_null() {
            return Err(std::io::Error::last_os_error().to_string());
        }
        drop(Handle(handle));
    }
    let handle = open(NAME, SERVICE_START)?.ok_or("Service introuvable après création.")?;
    if unsafe { StartServiceW(handle.0, 0, std::ptr::null()) } == 0 {
        let err = std::io::Error::last_os_error();
        if err.raw_os_error() != Some(ERROR_SERVICE_ALREADY_RUNNING as i32) {
            return Err(err.to_string());
        }
    }
    drop(handle);
    let result = wait_for(
        &WindowsServices,
        NAME,
        expected,
        Some(SERVICE_RUNNING),
        Duration::from_secs(15),
    );
    if let Err(err) = result {
        return match stop_owned(&WindowsServices, NAME, expected) {
            Ok(()) => Err(err),
            Err(cleanup) => Err(format!("{err} Arrêt après échec : {cleanup}")),
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct Fake {
        info: RefCell<Option<Info>>,
        calls: RefCell<Vec<String>>,
        fail_stop: bool,
        fail_delete: bool,
    }
    impl Backend for Fake {
        fn query(&self, _: &str) -> Result<Option<Info>, String> {
            Ok(self.info.borrow().clone())
        }
        fn stop(&self, name: &str) -> Result<(), String> {
            self.calls.borrow_mut().push(format!("stop {name}"));
            if self.fail_stop {
                return Err("denied".into());
            }
            self.info.borrow_mut().as_mut().unwrap().state = SERVICE_STOPPED;
            Ok(())
        }
        fn delete(&self, name: &str) -> Result<(), String> {
            self.calls.borrow_mut().push(format!("delete {name}"));
            if self.fail_delete {
                return Err("denied".into());
            }
            *self.info.borrow_mut() = None;
            Ok(())
        }
    }
    fn fake(path: &Path) -> Fake {
        Fake {
            info: RefCell::new(Some(Info {
                command: quote_argument(&path.to_string_lossy()),
                state: SERVICE_RUNNING,
                exit_code: 0,
            })),
            calls: RefCell::new(vec![]),
            fail_stop: false,
            fail_delete: false,
        }
    }
    #[test]
    fn refuses_foreign_service_and_never_touches_shared_driver() {
        let expected = std::env::current_exe().unwrap();
        let foreign = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let backend = fake(&foreign);
        assert!(uninstall_owned(&backend, NAME, std::slice::from_ref(&expected)).is_err());
        assert!(backend.calls.borrow().is_empty());
        let backend = fake(&expected);
        uninstall_owned(&backend, NAME, &[expected]).unwrap();
        assert_eq!(
            *backend.calls.borrow(),
            ["stop GoodByeFirewall", "delete GoodByeFirewall"]
        );
    }
    #[test]
    fn stop_and_delete_errors_are_reported_and_do_not_continue() {
        let expected = std::env::current_exe().unwrap();
        let mut backend = fake(&expected);
        backend.fail_stop = true;
        assert!(uninstall_owned(&backend, NAME, std::slice::from_ref(&expected)).is_err());
        assert_eq!(*backend.calls.borrow(), ["stop GoodByeFirewall"]);
        let mut backend = fake(&expected);
        backend.fail_delete = true;
        assert!(uninstall_owned(&backend, NAME, &[expected]).is_err());
        assert!(backend.info.borrow().is_some());
    }
    #[test]
    fn deletion_timeout_is_not_reported_as_success() {
        let expected = std::env::current_exe().unwrap();
        assert!(wait_for(&fake(&expected), NAME, &[expected], None, Duration::ZERO).is_err());
    }
    #[test]
    fn startup_exit_is_not_running() {
        let expected = std::env::current_exe().unwrap();
        let backend = fake(&expected);
        backend.info.borrow_mut().as_mut().unwrap().state = SERVICE_STOPPED;
        assert!(wait_for(
            &backend,
            NAME,
            &[expected],
            Some(SERVICE_RUNNING),
            Duration::ZERO
        )
        .is_err());
    }
}
