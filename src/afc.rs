use napi::bindgen_prelude::Buffer;
use napi::Result;
use napi_derive::napi;
use tokio::sync::Mutex;

use idevice::services::afc::opcode::AfcFopenMode;
use idevice::services::afc::AfcClient;
use idevice::IdeviceService;

use crate::helpers::{create_provider, to_napi_err};

#[napi(object)]
pub struct FileInfo {
    pub size: i64,
    pub blocks: i64,
    pub creation: String,
    pub modified: String,
    pub nlink: String,
    pub ifmt: String,
    pub link_target: Option<String>,
}

#[napi(object)]
pub struct DeviceStorageInfo {
    pub model: String,
    pub total_bytes: i64,
    pub free_bytes: i64,
    pub block_size: i64,
}

#[napi]
pub struct Afc {
    inner: Mutex<Option<AfcClient>>,
}

#[napi]
impl Afc {
    #[napi(factory)]
    pub async fn connect(udid: String) -> Result<Self> {
        let provider = create_provider(&udid).await?;
        let client = AfcClient::connect(&provider).await.map_err(to_napi_err)?;
        Ok(Self {
            inner: Mutex::new(Some(client)),
        })
    }

    #[napi]
    pub async fn read_directory(&self, path: String) -> Result<Vec<String>> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client.list_dir(path).await.map_err(to_napi_err)
    }

    #[napi]
    pub async fn read_file(&self, path: String) -> Result<Buffer> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        let mut fd = client.open(&path, AfcFopenMode::RdOnly).await.map_err(to_napi_err)?;
        let data = fd.read_entire().await.map_err(to_napi_err)?;
        fd.close().await.map_err(to_napi_err)?;
        Ok(Buffer::from(data))
    }

    #[napi]
    pub async fn write_file(&self, path: String, data: Buffer) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        let mut fd = client.open(&path, AfcFopenMode::WrOnly).await.map_err(to_napi_err)?;
        fd.write_entire(&data).await.map_err(to_napi_err)?;
        fd.close().await.map_err(to_napi_err)?;
        Ok(())
    }

    #[napi]
    pub async fn make_directory(&self, path: String) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client.mk_dir(path).await.map_err(to_napi_err)
    }

    #[napi]
    pub async fn remove(&self, path: String) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client.remove(path).await.map_err(to_napi_err)
    }

    #[napi]
    pub async fn remove_all(&self, path: String) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client.remove_all(path).await.map_err(to_napi_err)
    }

    #[napi]
    pub async fn rename(&self, from: String, to: String) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        client.rename(from, to).await.map_err(to_napi_err)
    }

    #[napi]
    pub async fn get_file_info(&self, path: String) -> Result<FileInfo> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        let info = client.get_file_info(path).await.map_err(to_napi_err)?;
        Ok(FileInfo {
            size: info.size as i64,
            blocks: info.blocks as i64,
            creation: info.creation.to_string(),
            modified: info.modified.to_string(),
            nlink: info.st_nlink,
            ifmt: info.st_ifmt,
            link_target: info.st_link_target,
        })
    }

    #[napi]
    pub async fn get_device_info(&self) -> Result<DeviceStorageInfo> {
        let mut guard = self.inner.lock().await;
        let client = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Connection closed"))?;
        let info = client.get_device_info().await.map_err(to_napi_err)?;
        Ok(DeviceStorageInfo {
            model: info.model,
            total_bytes: info.total_bytes as i64,
            free_bytes: info.free_bytes as i64,
            block_size: info.block_size as i64,
        })
    }

    #[napi]
    pub async fn close(&self) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let _ = guard.take();
        Ok(())
    }
}
