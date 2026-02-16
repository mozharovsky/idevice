import { describe, expect, it } from "vitest";
import {
  Afc,
  CompanionProxy,
  InstallationProxy,
  listDevices,
  Lockdown,
  Screenshotr,
  SyslogRelay,
} from "../index.js";

describe("idevice", () => {
  it("exports all expected symbols", () => {
    expect(typeof listDevices).toBe("function");
    expect(typeof Lockdown).toBe("function");
    expect(typeof InstallationProxy).toBe("function");
    expect(typeof Screenshotr).toBe("function");
    expect(typeof Afc).toBe("function");
    expect(typeof SyslogRelay).toBe("function");
    expect(typeof CompanionProxy).toBe("function");
  });

  it("listDevices returns an array", async () => {
    let devices;
    try {
      devices = await listDevices();
    } catch {
      // usbmuxd may not be available on CI runners
      console.log("usbmuxd not available, skipping listDevices test");
      return;
    }
    expect(Array.isArray(devices)).toBe(true);
    for (const d of devices) {
      expect(d).toHaveProperty("udid");
      expect(d).toHaveProperty("connectionType");
      expect(d).toHaveProperty("deviceId");
    }
  });

  // The following tests require a real iOS device connected via USB.
  // They are skipped automatically when no device is present.
  describe("with connected device", async () => {
    let devices = [];
    try {
      devices = await listDevices();
    } catch {
      // usbmuxd not available on this runner
    }
    const skip = devices.length === 0;
    const udid = devices[0]?.udid;

    it.skipIf(skip)("Lockdown.connect + getValue", async () => {
      const lockdown = await Lockdown.connect(udid);
      try {
        const name = await lockdown.getValue("DeviceName");
        expect(typeof name).toBe("string");

        const version = await lockdown.getValue("ProductVersion");
        expect(typeof version).toBe("string");
      } finally {
        await lockdown.close();
      }
    });

    it.skipIf(skip)("Screenshotr.connect + take", async () => {
      let screenshot;
      try {
        screenshot = await Screenshotr.connect(udid);
      } catch (e) {
        // Service may be unavailable if device is locked
        console.log("Screenshotr not available (device may be locked):", e.message);
        return;
      }
      try {
        const buffer = await screenshot.take();
        expect(buffer).toBeInstanceOf(Buffer);
        expect(buffer.length).toBeGreaterThan(0);
      } finally {
        await screenshot.close();
      }
    });

    it.skipIf(skip)("Afc.connect + readDirectory", async () => {
      const afc = await Afc.connect(udid);
      try {
        const files = await afc.readDirectory("/");
        expect(Array.isArray(files)).toBe(true);
        expect(files.length).toBeGreaterThan(0);
      } finally {
        await afc.close();
      }
    });

    it.skipIf(skip)("Afc.getDeviceInfo", async () => {
      const afc = await Afc.connect(udid);
      try {
        const info = await afc.getDeviceInfo();
        expect(info).toHaveProperty("model");
        expect(info).toHaveProperty("totalBytes");
        expect(info).toHaveProperty("freeBytes");
      } finally {
        await afc.close();
      }
    });

    it.skipIf(skip)("InstallationProxy.connect + getApps", async () => {
      const installer = await InstallationProxy.connect(udid);
      try {
        const apps = await installer.getApps("User");
        expect(typeof apps).toBe("object");
      } finally {
        await installer.close();
      }
    });
  });
});
