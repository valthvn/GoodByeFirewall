use std::io;
use std::process::Output;

pub fn command_output(result: io::Result<Output>) -> Result<String, String> {
    let output = result.map_err(|err| err.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if output.status.success() {
        Ok(stdout.into_owned())
    } else {
        Err(format!("{}{}", stdout, stderr).trim().to_owned())
    }
}

pub fn is_running(output: &str) -> bool {
    output.lines().any(|line| {
        let Some((key, value)) = line.split_once(':') else { return false; };
        matches!(key.trim(), "STATE" | "ETAT")
            && value.split_whitespace().next() == Some("4")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::windows::process::ExitStatusExt;
    use std::process::ExitStatus;

    #[test]
    fn running_state_supports_english_and_french() {
        assert!(is_running("STATE : 4 RUNNING"));
        assert!(is_running("ETAT : 4 RUNNING"));
        assert!(!is_running("SERVICE_NAME: RUNNING\nSTATE : 1 STOPPED"));
        assert!(!is_running("STATE : 2 START_PENDING"));
        assert!(!is_running("[SC] OpenService FAILED 1060"));
    }

    #[test]
    fn failed_service_commands_preserve_stdout_diagnostics() {
        let output = Output {
            status: ExitStatus::from_raw(5),
            stdout: b"[SC] Access denied".to_vec(),
            stderr: vec![],
        };
        assert_eq!(command_output(Ok(output)), Err("[SC] Access denied".into()));
        assert!(command_output(Err(io::Error::new(io::ErrorKind::NotFound, "sc missing"))).is_err());
    }
}
