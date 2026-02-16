use napi::Result;
use napi_derive::napi;
use tokio::sync::Mutex;

use idevice::services::installation_proxy::InstallationProxyClient;
use idevice::IdeviceService;

use crate::helpers::{create_provider, plist_to_json, to_napi_err};

#[napi]
pub struct InstallationProxy {
    inner: Mutex<Option<InstallationProxyClient>>,
}

#[napi]
impl InstallationProxy {
    #[napi(factory)]
    pub async fn connect(udid: String) -> Result<Self> {
        let provider = create_provider(&udid).await?;
        let client = InstallationProxyClient::connect(&provider).await.map_err(to_napi_err)?;
        Ok(Self {
            inner: Mutex::new(Some(client)),
        })
    }

    /// List installed apps. `app_type` can be "System", "User", or "Any".
    #[napi]
    pub async fn get_apps(&self, app_type: Option<String>) -> Result<serde_json::Value> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        let apps = client.get_apps(app_type.as_deref(), None).await.map_err(to_napi_err)?;

        let map: serde_json::Map<String, serde_json::Value> =
            apps.into_iter().map(|(k, v)| (k, plist_to_json(v))).collect();
        Ok(serde_json::Value::Object(map))
    }

    #[napi]
    pub async fn install(&self, package_path: String) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client.install(package_path, None).await.map_err(to_napi_err)?;
        Ok(())
    }

    #[napi]
    pub async fn uninstall(&self, bundle_id: String) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client.uninstall(bundle_id, None).await.map_err(to_napi_err)?;
        Ok(())
    }

    #[napi]
    pub async fn upgrade(&self, package_path: String) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client.upgrade(package_path, None).await.map_err(to_napi_err)?;
        Ok(())
    }

    #[napi]
    pub async fn close(&self) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let _ = guard.take();
        Ok(())
    }
}
