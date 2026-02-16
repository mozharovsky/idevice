use napi::Result;
use napi_derive::napi;
use tokio::sync::Mutex;

use idevice::provider::IdeviceProvider;
use idevice::services::lockdown::LockdownClient;
use idevice::IdeviceService;

use crate::helpers::{create_provider, json_to_plist, plist_to_json, to_napi_err};

#[napi]
pub struct Lockdown {
    inner: Mutex<Option<LockdownClient>>,
}

#[napi]
impl Lockdown {
    #[napi(factory)]
    pub async fn connect(udid: String) -> Result<Self> {
        let provider = create_provider(&udid).await?;
        let mut client = LockdownClient::connect(&provider).await.map_err(to_napi_err)?;
        let pairing = provider.get_pairing_file().await.map_err(to_napi_err)?;
        client.start_session(&pairing).await.map_err(to_napi_err)?;
        Ok(Self {
            inner: Mutex::new(Some(client)),
        })
    }

    #[napi]
    pub async fn get_value(&self, key: String) -> Result<serde_json::Value> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        let value = client.get_value(Some(&key), None).await.map_err(to_napi_err)?;
        Ok(plist_to_json(value))
    }

    #[napi]
    pub async fn get_value_with_domain(&self, key: String, domain: String) -> Result<serde_json::Value> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        let value = client.get_value(Some(&key), Some(&domain)).await.map_err(to_napi_err)?;
        Ok(plist_to_json(value))
    }

    #[napi]
    pub async fn get_all_values(&self) -> Result<serde_json::Value> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        let value = client.get_value(None, None).await.map_err(to_napi_err)?;
        Ok(plist_to_json(value))
    }

    #[napi]
    pub async fn set_value(&self, key: String, value: serde_json::Value) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client
            .set_value(key, json_to_plist(value), None)
            .await
            .map_err(to_napi_err)?;
        Ok(())
    }

    #[napi]
    pub async fn close(&self) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let _ = guard.take();
        Ok(())
    }
}
