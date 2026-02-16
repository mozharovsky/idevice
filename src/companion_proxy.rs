use napi::Result;
use napi_derive::napi;
use tokio::sync::Mutex;

use idevice::services::companion_proxy::CompanionProxy as CompanionProxyClient;
use idevice::IdeviceService;

use crate::helpers::{create_provider, plist_to_json, to_napi_err};

#[napi]
pub struct CompanionProxy {
    inner: Mutex<Option<CompanionProxyClient>>,
}

#[napi]
impl CompanionProxy {
    #[napi(factory)]
    pub async fn connect(udid: String) -> Result<Self> {
        let provider = create_provider(&udid).await?;
        let client = CompanionProxyClient::connect(&provider).await.map_err(to_napi_err)?;
        Ok(Self {
            inner: Mutex::new(Some(client)),
        })
    }

    /// List paired Apple Watch UDIDs.
    #[napi]
    pub async fn get_device_registry(&self) -> Result<Vec<String>> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client.get_device_registry().await.map_err(to_napi_err)
    }

    /// Get a value from a paired watch.
    #[napi]
    pub async fn get_value(&self, watch_udid: String, key: String) -> Result<serde_json::Value> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        let value = client.get_value(watch_udid, key).await.map_err(to_napi_err)?;
        Ok(plist_to_json(value))
    }

    /// Forward a service port from the paired Apple Watch through the iPhone.
    /// Returns the local port on the iPhone that tunnels to the watch service.
    /// Use this to reach watch services (e.g. installation_proxy) via the phone.
    #[napi]
    pub async fn start_forwarding_service_port(&self, port: u32, service_name: Option<String>) -> Result<u32> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        let forwarded = client
            .start_forwarding_service_port(port as u16, service_name.as_deref(), None)
            .await
            .map_err(to_napi_err)?;
        Ok(forwarded as u32)
    }

    /// Stop forwarding a previously forwarded service port.
    #[napi]
    pub async fn stop_forwarding_service_port(&self, port: u32) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client
            .stop_forwarding_service_port(port as u16)
            .await
            .map_err(to_napi_err)
    }

    #[napi]
    pub async fn close(&self) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let _ = guard.take();
        Ok(())
    }
}
