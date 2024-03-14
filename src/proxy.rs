use create::Proxy;

use strict_encoding::StrictSerialize;
   
#[derive(Debug, Clone, Eq, PartialEq, Display, From, Error)]
#[display(doc_comments)]
pub enum ProxyServerError {
    /// I/O or connectivity error. {0}
    IO(String),
    /// Server connectivity error. {0}
    Server(String),
    /// JSON RPC Parse error. {0}
    Parse(String),
    /// All endpoints failed error
    AllEndpointsFailed,
}

#[cfg(not(target_arch = "wasm32"))]
pub use server::{
    handle_file, proxy_consig_retrieve, proxy_consig_store, proxy_media_retrieve, proxy_media_store,
};
#[cfg(not(target_arch = "wasm32"))]
mod server {
    use reqwest::multipart::{self, Part};
    use std::path::PathBuf;
    use tokio::fs;

    use crate::{
        constants::{NETWORK, RGB_PROXY_ENDPOINT},
        info,
        rgb::structs::{
            RgbProxyConsigFileReq, RgbProxyConsigReq, RgbProxyConsigRes, RgbProxyConsigUploadReq,
            RgbProxyConsigUploadRes, RgbProxyMediaFileReq, RgbProxyMediaReq, RgbProxyMediaRes,
            RgbProxyMediaUploadReq, RgbProxyMediaUploadRes,
        },
        util::{post_data, upload_data},
    };

    use super::ProxyServerError;

    pub async fn proxy_consig_store(
        wallet_id: &str,
        request: RgbProxyConsigFileReq,
    ) -> Result<RgbProxyConsigUploadRes, ProxyServerError> {
        let RgbProxyConsigFileReq {
            file_name,
            bytes,
            params,
        } = request;

        let filepath = handle_file(wallet_id, &file_name, bytes.len())
            .await
            .map_err(|op| ProxyServerError::Server(op.to_string()))?;

        fs::write(filepath.clone(), bytes)
            .await
            .map_err(|op| ProxyServerError::Server(op.to_string()))?;

        let request_data = RgbProxyConsigUploadReq {
            params,
            file_name: filepath.to_string_lossy().to_string(),
        };

        let resp = fetch_consignment_post(request_data).await?;
        fs::remove_file(filepath)
            .await
            .map_err(|op| ProxyServerError::Server(op.to_string()))?;

        Ok(resp)
    }

    pub async fn proxy_media_store(
        wallet_id: &str,
        request: RgbProxyMediaFileReq,
    ) -> Result<RgbProxyMediaUploadRes, ProxyServerError> {
        let RgbProxyMediaFileReq {
            file_name,
            bytes,
            params,
        } = request;

        let filepath = handle_file(wallet_id, &file_name, bytes.len())
            .await
            .map_err(|op| ProxyServerError::IO(op.to_string()))?;

        fs::write(filepath.clone(), bytes)
            .await
            .map_err(|op| ProxyServerError::IO(op.to_string()))?;

        let request_data = RgbProxyMediaUploadReq {
            params,
            file_name: filepath.to_string_lossy().to_string(),
        };

        let resp = fetch_media_post(request_data).await?;
        fs::remove_file(filepath)
            .await
            .map_err(|op| ProxyServerError::IO(op.to_string()))?;

        Ok(resp)
    }

    pub async fn proxy_consig_retrieve(
        wallet_id: &str,
        request_id: &str,
    ) -> Result<Option<RgbProxyConsigRes>, ProxyServerError> {
        fetch_consignment_get(wallet_id, request_id).await
    }

    pub async fn proxy_media_retrieve(
        wallet_id: &str,
        attachment_id: &str,
    ) -> Result<Option<RgbProxyMediaRes>, ProxyServerError> {
        fetch_media_get(wallet_id, attachment_id).await
    }

    pub async fn handle_file(
        wallet_id: &str,
        name: &str,
        bytes: usize,
    ) -> Result<PathBuf, ProxyServerError> {
        let mut final_name = name.to_string();
        let network = NETWORK.read().await.to_string();
        let networks = ["bitcoin", "testnet", "signet", "regtest"];
        if !networks.into_iter().any(|x| name.contains(x)) {
            final_name = format!("{network}-{name}");
        }

        let filepath = std::path::Path::new(
            &std::env::var("RGB_PROXY_DIR").unwrap_or("/tmp/bitmaskd/proxy".to_owned()),
        )
        .join(format!("{wallet_id}/{final_name}"));

        let filedir = filepath.parent().unwrap();
        fs::create_dir_all(filedir)
            .await
            .map_err(|op| ProxyServerError::IO(op.to_string()))?;

        if bytes == 0 {
            info!(format!("read {}", filepath.to_string_lossy()));
        } else {
            info!(format!(
                "write {bytes} bytes to {}",
                filepath.to_string_lossy()
            ));
        }

        Ok(filepath)
    }

    async fn fetch_consignment_post(
        request_data: RgbProxyConsigUploadReq,
    ) -> Result<RgbProxyConsigUploadRes, ProxyServerError> {
        // Implement this
    }

    async fn fetch_consignment_get(
        wallet_id: &str,
        request_id: &str,
    ) -> Result<Option<RgbProxyConsigRes>, ProxyServerError> {
        // Implement this
    }

    async fn fetch_media_post(
        request_data: RgbProxyMediaUploadReq,
    ) -> Result<RgbProxyMediaUploadRes, ProxyServerError> {
        // Implement this
    }

    async fn fetch_media_get(
        wallet_id: &str,
        attachment_id: &str,
    ) -> Result<Option<RgbProxyMediaRes>, ProxyServerError> {
        // Implement this
    }
}

#[cfg(target_arch = "wasm32")]
pub use client::{
    proxy_consig_retrieve, proxy_consig_store, proxy_media_retrieve, proxy_media_store,
};

#[cfg(target_arch = "wasm32")]
mod client {
    // Implement client methods for wasm32 target
}
