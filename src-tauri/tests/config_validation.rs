use app_lib::config::{build_arguments, parse_arguments, quote_argument, BypassConfig};

fn config() -> BypassConfig {
    serde_json::from_value(serde_json::json!({
        "preset":"-5", "dns":"custom", "customDnsIp":"1.1.1.1", "customDnsPort":"53",
        "language":"fr", "theme":"light", "ttl":"none"
    }))
    .unwrap()
}

#[test]
fn invalid_dns_is_rejected_instead_of_silently_rewritten() {
    for invalid in ["999.1.1.1", "1x.1.1.1", "::1", "", "1.1.1.1 --evil"] {
        let mut cfg = config();
        cfg.custom_dns_ip = Some(invalid.into());
        assert!(build_arguments(&cfg).is_err(), "accepted {invalid}");
    }
}

#[test]
fn dns_port_must_be_nonzero_and_fit_u16() {
    for port in ["0", "65536", "5x3", "-1", ""] {
        let mut cfg = config();
        cfg.custom_dns_port = Some(port.into());
        assert!(build_arguments(&cfg).is_err(), "accepted {port}");
    }
    let mut cfg = config();
    cfg.custom_dns_port = Some("65535".into());
    assert!(build_arguments(&cfg).is_ok());
}

#[test]
fn arguments_roundtrip_spaces_quotes_and_trailing_backslashes() {
    let args = [
        "",
        "--blacklist",
        r"C:\My lists\blocked.txt",
        r"C:\directory with spaces\",
        "a\"b",
        "&|;%",
    ];
    let serialized = args
        .iter()
        .map(|arg| quote_argument(arg))
        .collect::<Vec<_>>()
        .join(" ");
    assert_eq!(parse_arguments(&serialized).unwrap(), args);
}

#[test]
fn malformed_quotes_and_control_characters_are_rejected() {
    assert!(parse_arguments("--blacklist \"unterminated").is_err());
    assert!(parse_arguments("--flag\n--evil").is_err());
    assert!(parse_arguments("a\0b").is_err());
}

#[test]
fn custom_args_preserve_windows_paths() {
    let mut cfg = config();
    cfg.preset = "custom".into();
    cfg.extra_args = Some(r#"--blacklist "C:\My lists\blocked.txt" -5"#.into());
    assert_eq!(
        build_arguments(&cfg).unwrap(),
        ["--blacklist", r"C:\My lists\blocked.txt", "-5"]
    );
}

#[test]
fn invalid_preset_ttl_and_resolver_are_rejected() {
    let mut cfg = config();
    cfg.preset = "--anything".into();
    assert!(build_arguments(&cfg).is_err());
    cfg.preset = "-5".into();
    cfg.ttl = Some("--set-ttl 0".into());
    assert!(build_arguments(&cfg).is_err());
    cfg.ttl = Some("--set-ttl 256".into());
    assert!(build_arguments(&cfg).is_err());
    cfg.ttl = Some("none".into());
    cfg.dns = Some("typo".into());
    assert!(build_arguments(&cfg).is_err());
}
