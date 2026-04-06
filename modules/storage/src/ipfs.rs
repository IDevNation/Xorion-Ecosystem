//! IPFS HTTP API client.
//!
//! Talks to an IPFS node (local or remote gateway) via its HTTP API.
//! No heavy libp2p dependency — just plain reqwest.

use crate::{Result, StorageError};
use serde::Deserialize;
use tracing::info;

/// Response from `POST /api/v0/add`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct AddResponse {
    hash: String,
    size: String,
}

/// A single link returned by `POST /api/v0/ls`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct LsLink {
    hash: String,
    name: String,
    size: u64,
}

/// Object wrapper returned by `POST /api/v0/ls`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct LsObject {
    links: Vec<LsLink>,
}

/// Top-level response from `POST /api/v0/ls`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct LsResponse {
    objects: Vec<LsObject>,
}

/// IPFS file entry returned by [`IpfsClient::ls`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IpfsEntry {
    pub cid: String,
    pub name: String,
    pub size: u64,
}

/// Client for the IPFS HTTP API.
pub struct IpfsClient {
    api_url: String,
    gateway_url: String,
    client: reqwest::Client,
    timeout: std::time::Duration,
}

impl IpfsClient {
    /// Connect to a local IPFS node at `http://127.0.0.1:5001`.
    pub fn new() -> Self {
        Self::with_api_url("http://127.0.0.1:5001")
    }

    /// Connect to a custom IPFS API endpoint.
    pub fn with_api_url(api_url: &str) -> Self {
        Self {
            api_url: api_url.trim_end_matches('/').to_string(),
            gateway_url: "https://ipfs.io/ipfs".to_string(),
            client: reqwest::Client::new(),
            timeout: std::time::Duration::from_secs(30),
        }
    }

    /// Set a custom public gateway URL for read-only fetches.
    pub fn with_gateway(mut self, gateway_url: &str) -> Self {
        self.gateway_url = gateway_url.trim_end_matches('/').to_string();
        self
    }

    /// Set request timeout.
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Return the configured API URL.
    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    /// Return the configured gateway URL.
    pub fn gateway_url(&self) -> &str {
        &self.gateway_url
    }

    /// Upload data to IPFS. Returns the CID.
    pub async fn add(&self, data: &[u8]) -> Result<String> {
        info!("Uploading {} bytes to IPFS", data.len());
        let url = format!("{}/api/v0/add", self.api_url);

        let part = reqwest::multipart::Part::bytes(data.to_vec()).file_name("data");
        let form = reqwest::multipart::Form::new().part("file", part);

        let resp: AddResponse = self
            .client
            .post(&url)
            .multipart(form)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| StorageError::Ipfs(format!("add request failed: {e}")))?
            .json()
            .await
            .map_err(|e| StorageError::Ipfs(format!("add response parse failed: {e}")))?;

        info!("Uploaded to IPFS: CID={}", resp.hash);
        Ok(resp.hash)
    }

    /// Download data from IPFS by CID (tries local node first, then gateway).
    pub async fn cat(&self, cid: &str) -> Result<Vec<u8>> {
        info!("Fetching CID {cid} from IPFS");

        // Try local API first
        let url = format!("{}/api/v0/cat?arg={cid}", self.api_url);
        match self
            .client
            .post(&url)
            .timeout(self.timeout)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                let bytes = resp
                    .bytes()
                    .await
                    .map_err(|e| StorageError::Ipfs(format!("read body failed: {e}")))?;
                return Ok(bytes.to_vec());
            }
            _ => {}
        }

        // Fallback to public gateway
        let gw_url = format!("{}/{cid}", self.gateway_url);
        let bytes = self
            .client
            .get(&gw_url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| StorageError::Ipfs(format!("gateway fetch failed: {e}")))?
            .bytes()
            .await
            .map_err(|e| StorageError::Ipfs(format!("gateway read failed: {e}")))?;

        Ok(bytes.to_vec())
    }

    /// Pin a CID so the local node keeps it.
    pub async fn pin_add(&self, cid: &str) -> Result<()> {
        let url = format!("{}/api/v0/pin/add?arg={cid}", self.api_url);
        self.client
            .post(&url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| StorageError::Ipfs(format!("pin add failed: {e}")))?;
        info!("Pinned CID {cid}");
        Ok(())
    }

    /// Unpin a CID.
    pub async fn pin_rm(&self, cid: &str) -> Result<()> {
        let url = format!("{}/api/v0/pin/rm?arg={cid}", self.api_url);
        self.client
            .post(&url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| StorageError::Ipfs(format!("pin rm failed: {e}")))?;
        info!("Unpinned CID {cid}");
        Ok(())
    }

    /// List directory contents of a CID.
    pub async fn ls(&self, cid: &str) -> Result<Vec<IpfsEntry>> {
        let url = format!("{}/api/v0/ls?arg={cid}", self.api_url);
        let resp: LsResponse = self
            .client
            .post(&url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| StorageError::Ipfs(format!("ls failed: {e}")))?
            .json()
            .await
            .map_err(|e| StorageError::Ipfs(format!("ls parse failed: {e}")))?;

        let entries = resp
            .objects
            .into_iter()
            .flat_map(|obj| {
                obj.links.into_iter().map(|link| IpfsEntry {
                    cid: link.hash,
                    name: link.name,
                    size: link.size,
                })
            })
            .collect();

        Ok(entries)
    }
}

impl Default for IpfsClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_default_urls() {
        let client = IpfsClient::new();
        assert_eq!(client.api_url(), "http://127.0.0.1:5001");
        assert!(client.gateway_url().contains("ipfs.io"));
    }

    #[test]
    fn custom_api_url() {
        let client = IpfsClient::with_api_url("http://localhost:9001");
        assert_eq!(client.api_url(), "http://localhost:9001");
    }

    #[test]
    fn custom_gateway() {
        let client = IpfsClient::new().with_gateway("https://gateway.pinata.cloud/ipfs");
        assert!(client.gateway_url().contains("pinata"));
    }

    #[test]
    fn custom_timeout() {
        let client = IpfsClient::new().with_timeout(std::time::Duration::from_secs(60));
        assert_eq!(client.timeout, std::time::Duration::from_secs(60));
    }

    #[test]
    fn trailing_slash_stripped() {
        let client = IpfsClient::with_api_url("http://localhost:5001/");
        assert_eq!(client.api_url(), "http://localhost:5001");
    }
}
