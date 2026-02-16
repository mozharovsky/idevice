#![deny(clippy::all)]

mod afc;
mod companion_proxy;
mod helpers;
mod installation_proxy;
mod lockdown;
mod screenshotr;
mod syslog_relay;

use napi::Result;
use napi_derive::napi;

use idevice::usbmuxd::{Connection, UsbmuxdConnection};

use crate::helpers::to_napi_err;

#[napi(object)]
pub struct DeviceInfo {
    pub udid: String,
    pub connection_type: String,
    pub device_id: u32,
}

#[napi]
pub async fn list_devices() -> Result<Vec<DeviceInfo>> {
    let mut conn = UsbmuxdConnection::default().await.map_err(to_napi_err)?;
    let devices = conn.get_devices().await.map_err(to_napi_err)?;

    Ok(devices
        .into_iter()
        .map(|d| DeviceInfo {
            udid: d.udid,
            connection_type: match d.connection_type {
                Connection::Usb => "usb".to_string(),
                Connection::Network(addr) => format!("network:{addr}"),
                Connection::Unknown(s) => s,
            },
            device_id: d.device_id,
        })
        .collect())
}
