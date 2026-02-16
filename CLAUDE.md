# CLAUDE.md — AI Context for @xcodekit/idevice

## What This Is

napi-rs bindings for the [idevice](https://github.com/jkcoxson/idevice) Rust crate (v0.1.x).
Exposes iOS device services to Node.js/TypeScript as `@xcodekit/idevice`.

## Build

```bash
npm install
npx napi build --platform --release   # or --debug for dev
npm test                               # vitest (requires connected device for integration tests)
```

## Architecture

- `src/lib.rs` — module declarations, `listDevices()`, `DeviceInfo`
- `src/helpers.rs` — `UsbmuxdProvider` construction, plist<->JSON conversion, error mapping
- `src/lockdown.rs` — `Lockdown` class (device info, start services)
- `src/installation_proxy.rs` — `InstallationProxy` class (install/uninstall apps)
- `src/screenshotr.rs` — `Screenshotr` class (take screenshots)
- `src/afc.rs` — `Afc` class (Apple File Conduit — filesystem access)
- `src/syslog_relay.rs` — `SyslogRelay` class (stream system logs)
- `src/companion_proxy.rs` — `CompanionProxy` class (Apple Watch management)

Each service wraps the upstream client in `tokio::sync::Mutex<Option<T>>` for:

- Interior mutability (napi async methods use `&self`)
- Graceful `close()` via `Option::take()`

## Connection Pattern

Every service's `connect(udid)` does:

1. `UsbmuxdAddr::default()` + `UsbmuxdConnection::default()` — connect to usbmuxd daemon
2. `conn.get_device(udid)` — resolve UDID
3. `device.to_provider(addr, label)` — build `UsbmuxdProvider`
4. `ServiceClient::connect(&provider)` — connect to the service (goes through lockdown internally)

For `Lockdown` specifically, steps are: connect to port 62078 → start TLS session with pairing file.

## Key Types

- plist `Value` ↔ JSON `Value` conversion in `helpers.rs`
- `Buffer` (napi) for binary data (screenshots, file contents)
- All async methods return `Promise<T>` in JS

## npm Package Structure

- `npm/idevice/` — main `@xcodekit/idevice` package
- `npm/idevice/platforms/` — per-platform binary packages (`@xcodekit/idevice-darwin-arm64`, etc.)
- `scripts/build-pkg.sh` — assembles `pkg/idevice/` for publishing
