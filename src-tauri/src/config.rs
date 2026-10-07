use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use windows_sys::Win32::{Foundation::LocalFree, UI::Shell::CommandLineToArgvW};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BypassConfig {
    #[serde(default = "default_preset")]
    pub preset: String,
    pub ttl: Option<String>,
    pub dns: Option<String>,
    pub custom_dns_ip: Option<String>,
    pub custom_dns_port: Option<String>,
    pub extra_args: Option<String>,
    #[serde(default = "default_lang")]
    pub language: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default)]
    pub is_service_mode: bool,
}

fn default_preset() -> String {
    "-5".into()
}
fn default_lang() -> String {
    "fr".into()
}
fn default_theme() -> String {
    "light".into()
}

impl Default for BypassConfig {
    fn default() -> Self {
        Self {
            preset: default_preset(),
            ttl: Some("none".into()),
            dns: Some("cloudflare".into()),
            custom_dns_ip: Some("1.1.1.1".into()),
            custom_dns_port: Some("53".into()),
            extra_args: Some(String::new()),
            language: default_lang(),
            theme: default_theme(),
            is_service_mode: false,
        }
    }
}

pub fn parse_arguments(input: &str) -> Result<Vec<String>, String> {
    if input.len() > 8192 || input.chars().any(char::is_control) {
        return Err("Arguments trop longs ou contenant des caractères de contrôle.".into());
    }
    let mut quoted = false;
    let mut slashes = 0;
    for ch in input.chars() {
        if ch == '\\' {
            slashes += 1;
            continue;
        }
        if ch == '"' && slashes % 2 == 0 {
            quoted = !quoted;
        }
        slashes = 0;
    }
    if quoted {
        return Err("Guillemets non fermés dans les arguments.".into());
    }
    let wide: Vec<u16> = format!("engine.exe {input}")
        .encode_utf16()
        .chain(Some(0))
        .collect();
    let mut count = 0;
    // Windows owns this allocation; convert while it is live and always release it.
    let argv = unsafe { CommandLineToArgvW(wide.as_ptr(), &mut count) };
    if argv.is_null() {
        return Err(std::io::Error::last_os_error().to_string());
    }
    let result = unsafe {
        std::slice::from_raw_parts(argv, count as usize)
            .iter()
            .skip(1)
            .map(|&arg| {
                let mut len = 0;
                while *arg.add(len) != 0 {
                    len += 1;
                }
                String::from_utf16_lossy(std::slice::from_raw_parts(arg, len))
            })
            .collect()
    };
    unsafe {
        LocalFree(argv.cast());
    }
    Ok(result)
}

pub fn quote_argument(arg: &str) -> String {
    let mut result = String::from("\"");
    let mut slashes = 0;
    for ch in arg.chars() {
        if ch == '\\' {
            slashes += 1;
            continue;
        }
        result.extend(std::iter::repeat('\\').take(if ch == '"' {
            slashes * 2 + 1
        } else {
            slashes
        }));
        result.push(ch);
        slashes = 0;
    }
    result.extend(std::iter::repeat('\\').take(slashes * 2));
    result.push('"');
    result
}

pub fn build_arguments(cfg: &BypassConfig) -> Result<Vec<String>, String> {
    if cfg.preset == "custom" {
        let args = parse_arguments(cfg.extra_args.as_deref().unwrap_or(""))?;
        return Ok(if args.is_empty() {
            vec!["-5".into()]
        } else {
            args
        });
    }
    if !matches!(
        cfg.preset.as_str(),
        "-1" | "-2" | "-3" | "-4" | "-5" | "-6" | "-7" | "-8" | "-9"
    ) {
        return Err("Le niveau DPI doit être compris entre -1 et -9, ou custom.".into());
    }
    let mut args = vec![cfg.preset.clone(), "--max-payload".into(), "1200".into()];
    let ttl = cfg.ttl.as_deref().unwrap_or("none");
    if ttl == "--auto-ttl" {
        args.push(ttl.into());
    } else if ttl != "none" {
        let parts = parse_arguments(ttl)?;
        if parts.len() != 2
            || parts[0] != "--set-ttl"
            || parts[1].parse::<u8>().ok().filter(|&v| v > 0).is_none()
        {
            return Err(
                "TTL invalide : --auto-ttl ou --set-ttl suivi d'une valeur de 1 à 255.".into(),
            );
        }
        args.extend(parts);
    }
    let dns = cfg.dns.as_deref().unwrap_or("none");
    let resolver = match dns {
        "none" => None,
        "cloudflare" => Some(("1.1.1.1", "2606:4700:4700::1111")),
        "quad9" => Some(("9.9.9.9", "2620:fe::fe")),
        "fdn" => Some(("80.67.169.12", "2001:910:800::12")),
        "adguard" => Some(("94.140.14.14", "2a10:50c0::ad1:ff")),
        "google" => Some(("8.8.8.8", "2001:4860:4860::8888")),
        "custom" => {
            let ip = cfg
                .custom_dns_ip
                .as_deref()
                .unwrap_or("")
                .trim()
                .parse::<Ipv4Addr>()
                .map_err(|_| {
                    "Le DNS personnalisé doit être une adresse IPv4 valide.".to_string()
                })?;
            let port = cfg
                .custom_dns_port
                .as_deref()
                .unwrap_or("53")
                .trim()
                .parse::<u16>()
                .ok()
                .filter(|&port| port != 0)
                .ok_or("Le port DNS doit être compris entre 1 et 65535.")?;
            args.extend([
                "--dns-addr".into(),
                ip.to_string(),
                "--dns-port".into(),
                port.to_string(),
            ]);
            None
        }
        _ => return Err("Résolveur DNS inconnu.".into()),
    };
    if let Some((v4, v6)) = resolver {
        args.extend(
            [
                "--dns-addr",
                v4,
                "--dns-port",
                "53",
                "--dnsv6-addr",
                v6,
                "--dnsv6-port",
                "53",
            ]
            .map(str::to_owned),
        );
    }
    args.extend(parse_arguments(cfg.extra_args.as_deref().unwrap_or(""))?);
    Ok(args)
}
