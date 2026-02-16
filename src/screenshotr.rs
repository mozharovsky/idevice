use napi::bindgen_prelude::Buffer;
use napi::Result;
use napi_derive::napi;
use tokio::sync::Mutex;

use idevice::services::screenshotr::ScreenshotService;
use idevice::IdeviceService;

use crate::helpers::{create_provider, to_napi_err};

#[napi]
pub struct Screenshotr {
    inner: Mutex<Option<ScreenshotService>>,
}

#[napi]
impl Screenshotr {
    #[napi(factory)]
    pub async fn connect(udid: String) -> Result<Self> {
        let provider = create_provider(&udid).await?;
        let client = ScreenshotService::connect(&provider).await.map_err(to_napi_err)?;
        Ok(Self {
            inner: Mutex::new(Some(client)),
        })
    }

    /// Take a screenshot. Returns raw image data (typically PNG or TIFF).
    #[napi]
    pub async fn take(&self) -> Result<Buffer> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        let bytes = client.take_screenshot().await.map_err(to_napi_err)?;
        Ok(Buffer::from(bytes))
    }

    #[napi]
    pub async fn close(&self) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let _ = guard.take();
        Ok(())
    }
}
