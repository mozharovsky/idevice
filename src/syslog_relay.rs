use napi::Result;
use napi_derive::napi;
use tokio::sync::Mutex;

use idevice::services::syslog_relay::SyslogRelayClient;
use idevice::IdeviceService;

use crate::helpers::{create_provider, to_napi_err};

#[napi]
pub struct SyslogRelay {
    inner: Mutex<Option<SyslogRelayClient>>,
}

#[napi]
impl SyslogRelay {
    #[napi(factory)]
    pub async fn connect(udid: String) -> Result<Self> {
        let provider = create_provider(&udid).await?;
        let client = SyslogRelayClient::connect(&provider).await.map_err(to_napi_err)?;
        Ok(Self {
            inner: Mutex::new(Some(client)),
        })
    }

    /// Read the next log line. Blocks until a line is available.
    #[napi]
    pub async fn next(&self) -> Result<String> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client.next().await.map_err(to_napi_err)
    }

    #[napi]
    pub async fn close(&self) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let _ = guard.take();
        Ok(())
    }
}
