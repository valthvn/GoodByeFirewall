use crate::{
    engine::{Engine, LogSink},
    paths, service,
};
use std::os::windows::process::CommandExt;
use std::{
    process::Command,
    sync::atomic::{AtomicBool, AtomicPtr, Ordering},
    time::Duration,
};
use windows_sys::Win32::System::Services::*;

static STOP: AtomicBool = AtomicBool::new(false);
static STATUS: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());

fn report(state: u32, error: u32) {
    let mut status = SERVICE_STATUS {
        dwServiceType: SERVICE_WIN32_OWN_PROCESS,
        dwCurrentState: state,
        dwControlsAccepted: if state == SERVICE_RUNNING {
            SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN
        } else {
            0
        },
        dwWin32ExitCode: error,
        dwWaitHint: if state == SERVICE_START_PENDING {
            15000
        } else {
            0
        },
        ..Default::default()
    };
    if state == SERVICE_START_PENDING {
        status.dwCheckPoint = 1;
    }
    unsafe {
        SetServiceStatus(STATUS.load(Ordering::Acquire), &status);
    }
}

unsafe extern "system" fn control(
    code: u32,
    _: u32,
    _: *mut std::ffi::c_void,
    _: *mut std::ffi::c_void,
) -> u32 {
    if code == SERVICE_CONTROL_STOP || code == SERVICE_CONTROL_SHUTDOWN {
        STOP.store(true, Ordering::Release);
    }
    0
}

unsafe extern "system" fn main(_: u32, _: *mut *mut u16) {
    let name = paths::wide(service::NAME);
    let status = RegisterServiceCtrlHandlerExW(name.as_ptr(), Some(control), std::ptr::null());
    if status.is_null() {
        return;
    }
    STATUS.store(status, Ordering::Release);
    report(SERVICE_START_PENDING, 0);
    let result = run_engine();
    report(SERVICE_STOPPED, if result.is_ok() { 0 } else { 1 });
}

fn run_engine() -> Result<(), String> {
    let host = std::env::current_exe().map_err(|err| err.to_string())?;
    if !cfg!(debug_assertions) {
        paths::require_protected(&host)?;
    }
    let resources = host.parent().ok_or("Répertoire de service introuvable.")?;
    if !cfg!(debug_assertions) {
        paths::require_protected(resources)?;
        paths::require_protected(&resources.join("BinTools"))?;
    }
    let exe = paths::engine_path(resources)?;
    let args: Vec<_> = std::env::args().skip(2).collect();
    let mut cmd = Command::new(&exe);
    cmd.args(args)
        .current_dir(exe.parent().unwrap())
        .creation_flags(0x08000000);
    let log: LogSink = std::sync::Arc::new(|line| {
        log::info!("{line}");
    });
    let mut engine = Engine::spawn(&mut cmd, log)?;
    engine.observe_filter(&exe.parent().unwrap().join("WinDivert.dll"))?;
    engine.wait_ready(Duration::from_secs(10))?;
    report(SERVICE_RUNNING, 0);
    while !STOP.load(Ordering::Acquire) {
        if !engine.alive()? {
            return Err("Le moteur s'est arrêté.".into());
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    report(SERVICE_STOP_PENDING, 0);
    engine.stop()
}

pub fn dispatch() -> Result<(), String> {
    let mut name = paths::wide(service::NAME);
    let table = [
        SERVICE_TABLE_ENTRYW {
            lpServiceName: name.as_mut_ptr(),
            lpServiceProc: Some(main),
        },
        SERVICE_TABLE_ENTRYW::default(),
    ];
    if unsafe { StartServiceCtrlDispatcherW(table.as_ptr()) } == 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    Ok(())
}
