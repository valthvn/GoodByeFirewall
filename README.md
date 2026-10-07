<div align="center">

# GoodByeFirewall

**Windows desktop client for Deep Packet Inspection (DPI) circumvention and censorship bypass.**  
Lightweight native UI built with Tauri v2, integrated system tray, and kernel-level WinDivert packet engine.

[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20(x64)-0078D6?style=flat-square&logo=windows)](https://github.com)
[![Engine](https://img.shields.io/badge/Engine-GoodbyeDPI%20%2B%20WinDivert-0071e3?style=flat-square)](https://github.com/ValdikSS/GoodbyeDPI)
[![License](https://img.shields.io/badge/License-MIT-orange?style=flat-square)](LICENSE)

</div>

---

## Overview

**GoodByeFirewall** runs locally on Windows to circumvent **Deep Packet Inspection (DPI)** systems, ISP-level website filtering, and domain blocks.

The packet manipulation engine is based on [GoodbyeDPI](https://github.com/ValdikSS/GoodbyeDPI) and uses the [WinDivert](https://reqrypt.org/windivert.html) network filtering driver. It modifies outgoing packets on the fly directly on your network stack—without routing your traffic through external proxy or VPN servers, preserving full connection speed and direct routing.

---

## How It Works

### Deep Packet Inspection Mechanics
Many ISP filters do not block destination IP addresses outright. Instead, intermediate appliances inspect network traffic in real time:
- **HTTP**: Inspects the plain-text `Host:` request header.
- **HTTPS (TLS)**: Although the session payload is encrypted, the initial **TLS ClientHello** contains the **Server Name Indication (SNI)** in clear text, revealing the target domain.

### Circumvention Techniques
GoodByeFirewall intercepts packets locally before they exit the network adapter:

1. **TCP Packet Fragmentation (`-f` & `-e`)**:
   Splits the HTTP request or TLS ClientHello packet into fragments (e.g. splitting across the SNI domain name). Most inspection middleboxes do not perform complete TCP stream reassembly for performance reasons, while the destination server reassembles the fragments normally.

2. **Out-of-Order TCP Segmentation**:
   Transmits the second segment of the ClientHello prior to the first. The destination server reorders packets per the TCP specification, while inspection devices that do not track state fail to detect the target domain.

3. **HTTP Header Formatting**:
   Adjusts whitespace and capitalization in HTTP headers (e.g., `host:` instead of `Host:`), bypassing pattern-matching rules while remaining valid HTTP.

4. **TTL Spoofing / Decoy Packets (`--set-ttl`)**:
   Sends fake request packets with a low Time-To-Live (TTL). The decoy packet reaches the ISP middlebox and poisons its filter state, but expires before reaching the destination server. The authentic packet follows immediately.

---

## Features

### Bypass Presets (-1 to -9)

| Preset | Mode | Description |
| :--- | :--- | :--- |
| **`-5`** | **Aggressive (Default)** | Recommended for most networks. Combines TCP segmentation, TLS SNI splitting, and header alterations. |
| **`-1`** | **Standard** | Conservative passive mode with standard fragmentation. |
| **`-2`** | **Quick ACK** | Suitable for networks requiring rapid TCP ACKs to avoid socket timeouts. |
| **`-3`** | **No Fragmentation** | Disables packet splitting. Suitable for lossy Wi-Fi or latency-sensitive applications. |
| **`-4`** | **Balanced** | Fast profile with minimal fragmentation. |
| **`-6` to `-9`** | **Advanced** | Variations with ACK timeouts and alternate record splitting for specific middleboxes. |
| **`Custom`** | **Manual Arguments** | Pass custom command-line flags directly to the daemon. |

### DNS Redirection
Redirects UDP port 53 DNS queries to prevent DNS hijacking and poisoning:
- **Cloudflare DNS** (`1.1.1.1` / `1.0.0.1`) — Default resolver
- **FDN DNS** (`80.67.169.12` / `2001:910:800::12`) — French non-profit uncensored resolver
- **AdGuard DNS** (`94.140.14.14`) — Ad-blocking resolver
- **Quad9** (`9.9.9.9`) — Privacy and threat-blocking resolver
- **Google Public DNS** (`8.8.8.8`) — High-availability resolver
- **Custom DNS** — Custom IPv4 address and port

### Operating Modes
- **Direct Session**: Spawns a background daemon (`goodbyefirewall-daemon.exe`) that runs alongside the application. Quitting the app stops only its own daemon. A Windows Job also cleans up that child if the UI process crashes.
- **Windows Service**: Installs the application as a native Windows service host (`--service`), providing automatic startup on boot without requiring the UI. The host verifies and monitors its daemon and reports RUNNING only after filter activation. Quitting the UI leaves the service running; use the protection toggle to stop it.

### Installation and legacy service migration

Production builds require a protected installation in **Program Files**. The NSIS installer now installs for all users. The daemon, WinDivert DLL and driver must match the hashes pinned in the client. Updating those binaries requires updating the pins after verifying their provenance. Debug builds use the fixed repository engine directory.

The app manages only its own child process and services whose executable belongs to this installation. It preserves the shared WinDivert driver. Use **Migrate legacy service** to remove an older `GoodbyeDPI` service belonging to this installation, then activate Windows Service mode. A service belonging to a different installation is refused and must be managed through that installation.

Administrator access is required for filtering. Remembered elevation is enabled only for protected executables; development or user-writable installations require UAC. The optional `setup-skip-uac.ps1` script also requires an elevated shell and refuses a foreign scheduled task.

Custom arguments follow Windows quoting rules; for example, `--blacklist "C:\My lists\blocked.txt"` preserves a path containing spaces. They are passed directly to the daemon, without a shell.

### System Tray & Notifications
- Closing the window minimizes the app to the Windows system tray.
- Tray context menu allows quick protection toggle, opening the main window, or quitting.
- Native Windows notifications indicate protection status changes.

---

## Building from Source

### Prerequisites
- Windows 10 or Windows 11 (64-bit)
- [Node.js](https://nodejs.org/) (v18+)
- [Rust toolchain](https://rustup.rs/) (stable-x86_64-pc-windows-msvc or stable-x86_64-pc-windows-gnu)

### Instructions
```bash
# Clone repository
git clone https://github.com/valthvn/GoodByeFirewall.git
cd GoodByeFirewall

# Install dependencies
npm install

# Run in development mode
npm run dev

# Build production binary and NSIS installer
npm run build
```

The compiled installer is output to:
`src-tauri/target/release/bundle/nsis/GoodByeFirewall_2.0.0_x64-setup.exe`

---

## Verification

`npm test` runs renderer regressions with simulated DOM/IPC. On Windows, run:

```powershell
cargo test --locked --manifest-path src-tauri/Cargo.toml
cargo clippy --locked --all-targets --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo fmt --manifest-path src-tauri/Cargo.toml --check
```

These tests do not modify system services or load the network driver. See [REVIEW.md](REVIEW.md) for findings, changes, measured renderer improvements and remaining native validation requirements.

## License & Attribution

- Released under the **MIT License**. See `LICENSE` for details.
- Packet manipulation engine and driver components originate from [GoodbyeDPI](https://github.com/ValdikSS/GoodbyeDPI) by ValdikSS and [WinDivert](https://reqrypt.org/windivert.html).
