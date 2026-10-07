# Changelog

## 2.0.0 — 2026-10-07

### Fixed
- Protection status follows the actual engine and service state, including failed starts and stops.
- Service startup, shutdown and removal report errors instead of false success.
- Concurrent operations are serialized; settings saves and console rendering are batched.
- Custom Windows paths containing spaces are preserved; DNS, port and TTL values are validated.

### Security
- Production engine files use fixed installation paths, pinned hashes and protected permissions.
- The renderer now uses a Content Security Policy; remembered elevation refuses writable installations.

### Changed
- Installation is now for all users in Program Files and requires administrator privileges.
- Quitting the UI preserves an independent Windows service. Shared WinDivert drivers and foreign processes/services are preserved.
- Use the explicit legacy-service migration action for an older service belonging to this installation. User-writable installations must be replaced with the new installer; their executable is no longer eligible for remembered elevation.

Automated tests do not cover live WinDivert filtering, startup at boot or WebView2 rendering.
