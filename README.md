# @xcodekit/idevice

Native Node.js bindings for iOS device services, powered by the [idevice](https://github.com/jkcoxson/idevice) Rust crate and [napi-rs](https://napi.rs/).

Communicate with iOS devices over USB — list devices, get device info, install apps, take screenshots, access the filesystem, stream logs, and more.

## Install

```bash
npm install @xcodekit/idevice
```

Platform-specific native binaries are installed automatically via optional dependencies.

**Supported platforms:** macOS (arm64, x64), Linux (x64, arm64), Windows (x64)

## Usage

```typescript
import {
  listDevices,
  Lockdown,
  InstallationProxy,
  Screenshotr,
  Afc,
  SyslogRelay,
} from "@xcodekit/idevice";

// List connected devices
const devices = await listDevices();
// [{ udid: "00008...", connectionType: "usb", deviceId: 3 }]

// Get device info
const lockdown = await Lockdown.connect(devices[0].udid);
const name = await lockdown.getValue("DeviceName");
const version = await lockdown.getValue("ProductVersion");
await lockdown.close();

// Install an app
const installer = await InstallationProxy.connect(udid);
await installer.install("/path/to/MyApp.ipa");
await installer.close();

// Take a screenshot
const screenshot = await Screenshotr.connect(udid);
const pngBuffer = await screenshot.take();
writeFileSync("screenshot.png", pngBuffer);
await screenshot.close();

// File system access
const afc = await Afc.connect(udid);
const files = await afc.readDirectory("/");
const data = await afc.readFile("/some/file.txt");
await afc.writeFile("/some/new.txt", Buffer.from("hello"));
await afc.close();

// Stream system logs
const syslog = await SyslogRelay.connect(udid);
while (true) {
  const line = await syslog.next();
  console.log(line);
}
```

## API

### `listDevices(): Promise<DeviceInfo[]>`

Returns all connected iOS devices.

### `Lockdown`

- `Lockdown.connect(udid): Promise<Lockdown>` — Connect and start authenticated session
- `.getValue(key): Promise<any>` — Get a device value (e.g. `"DeviceName"`, `"ProductVersion"`)
- `.getValueWithDomain(key, domain): Promise<any>` — Get a value from a specific domain
- `.getAllValues(): Promise<any>` — Get all device values
- `.setValue(key, value): Promise<void>` — Set a device value
- `.close(): Promise<void>`

### `InstallationProxy`

- `InstallationProxy.connect(udid): Promise<InstallationProxy>`
- `.getApps(appType?): Promise<object>` — List installed apps (`"User"`, `"System"`, `"Any"`)
- `.install(ipaPath): Promise<void>` — Install an IPA
- `.uninstall(bundleId): Promise<void>` — Uninstall an app
- `.upgrade(ipaPath): Promise<void>` — Upgrade an installed app
- `.close(): Promise<void>`

### `Screenshotr`

- `Screenshotr.connect(udid): Promise<Screenshotr>`
- `.take(): Promise<Buffer>` — Take a screenshot (returns image data)
- `.close(): Promise<void>`

### `Afc` (Apple File Conduit)

- `Afc.connect(udid): Promise<Afc>`
- `.readDirectory(path): Promise<string[]>`
- `.readFile(path): Promise<Buffer>`
- `.writeFile(path, data): Promise<void>`
- `.makeDirectory(path): Promise<void>`
- `.remove(path): Promise<void>`
- `.removeAll(path): Promise<void>` — Recursive remove
- `.rename(from, to): Promise<void>`
- `.getFileInfo(path): Promise<FileInfo>`
- `.getDeviceInfo(): Promise<DeviceStorageInfo>`
- `.close(): Promise<void>`

### `SyslogRelay`

- `SyslogRelay.connect(udid): Promise<SyslogRelay>`
- `.next(): Promise<string>` — Read next log line (blocks until available)
- `.close(): Promise<void>`

### `CompanionProxy`

- `CompanionProxy.connect(udid): Promise<CompanionProxy>`
- `.getDeviceRegistry(): Promise<string[]>` — List paired Apple Watch UDIDs
- `.getValue(watchUdid, key): Promise<any>`
- `.startForwardingServicePort(port, serviceName?): Promise<number>` — Forward a Watch service port through the iPhone; returns the local forwarded port
- `.stopForwardingServicePort(port): Promise<void>` — Stop a previously forwarded port
- `.close(): Promise<void>`

## Requirements

- An iOS device connected via USB
- `usbmuxd` running (automatic on macOS; install via package manager on Linux)
- Device must be paired/trusted

## License

MIT
