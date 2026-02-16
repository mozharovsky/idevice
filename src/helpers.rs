use idevice::provider::UsbmuxdProvider;
use idevice::usbmuxd::{UsbmuxdAddr, UsbmuxdConnection};
use idevice::IdeviceError;

pub fn to_napi_err(err: IdeviceError) -> napi::Error {
    napi::Error::from_reason(format!("{err}"))
}

pub async fn create_provider(udid: &str) -> napi::Result<UsbmuxdProvider> {
    let addr = UsbmuxdAddr::default();
    let mut conn = UsbmuxdConnection::default().await.map_err(to_napi_err)?;
    let device = conn.get_device(udid).await.map_err(to_napi_err)?;
    Ok(device.to_provider(addr, "idevice-napi"))
}

pub fn plist_to_json(value: plist::Value) -> serde_json::Value {
    match value {
        plist::Value::String(s) => serde_json::Value::String(s),
        plist::Value::Boolean(b) => serde_json::Value::Bool(b),
        plist::Value::Real(f) => serde_json::Number::from_f64(f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        plist::Value::Integer(i) => {
            if let Some(n) = i.as_signed() {
                serde_json::json!(n)
            } else if let Some(n) = i.as_unsigned() {
                serde_json::json!(n)
            } else {
                serde_json::Value::Null
            }
        }
        plist::Value::Data(bytes) => {
            use base64::Engine as _;
            serde_json::Value::String(base64::engine::general_purpose::STANDARD.encode(&bytes))
        }
        plist::Value::Date(d) => serde_json::Value::String(format!("{d:?}")),
        plist::Value::Array(arr) => serde_json::Value::Array(arr.into_iter().map(plist_to_json).collect()),
        plist::Value::Dictionary(dict) => {
            let map: serde_json::Map<String, serde_json::Value> =
                dict.into_iter().map(|(k, v)| (k, plist_to_json(v))).collect();
            serde_json::Value::Object(map)
        }
        plist::Value::Uid(uid) => serde_json::json!(uid.get()),
        _ => serde_json::Value::Null,
    }
}

pub fn json_to_plist(value: serde_json::Value) -> plist::Value {
    match value {
        serde_json::Value::Null => plist::Value::String(String::new()),
        serde_json::Value::Bool(b) => plist::Value::Boolean(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                plist::Value::Integer(i.into())
            } else if let Some(f) = n.as_f64() {
                plist::Value::Real(f)
            } else {
                plist::Value::String(n.to_string())
            }
        }
        serde_json::Value::String(s) => plist::Value::String(s),
        serde_json::Value::Array(arr) => plist::Value::Array(arr.into_iter().map(json_to_plist).collect()),
        serde_json::Value::Object(map) => {
            let dict: plist::Dictionary = map.into_iter().map(|(k, v)| (k, json_to_plist(v))).collect();
            plist::Value::Dictionary(dict)
        }
    }
}
