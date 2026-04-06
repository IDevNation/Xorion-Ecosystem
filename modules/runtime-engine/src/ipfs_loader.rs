//! IPFS loader for fetching WASM dApps from the decentralized web.

use anyhow::{anyhow, Context, Result};
use tracing::info;

/// Default public IPFS gateway.
const DEFAULT_GATEWAY: &str = "https://ipfs.io/ipfs";

/// Loader for fetching WASM modules from IPFS.
pub struct IpfsLoader {
    gateway: String,
    timeout: std::time::Duration,
}

impl IpfsLoader {
    /// Create a loader using the default public gateway.
    pub fn new() -> Self {
        Self {
            gateway: DEFAULT_GATEWAY.to_string(),
            timeout: std::time::Duration::from_secs(30),
        }
    }

    /// Use a custom IPFS gateway URL (without trailing slash).
    pub fn with_gateway(mut self, gateway: &str) -> Self {
        self.gateway = gateway.to_string();
        self
    }

    /// Override the download timeout.
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Fetch WASM bytes from IPFS by CID.
    pub async fn load(&self, cid: &str) -> Result<Vec<u8>> {
        let url = format!("{}/{}", self.gateway, cid);
        info!("Fetching dApp from IPFS: {url}");

        let client = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()
            .context("failed to build HTTP client")?;

        let response = client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("IPFS request failed: {url}"))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "IPFS gateway returned {}: {url}",
                response.status()
            ));
        }

        let bytes = response
            .bytes()
            .await
            .context("failed to read IPFS response body")?;

        info!("Loaded {} bytes from IPFS CID {cid}", bytes.len());
        Ok(bytes.to_vec())
    }

    /// Fetch from a local IPFS node (http://localhost:5001).
    pub async fn load_local(cid: &str) -> Result<Vec<u8>> {
        let url = format!("http://localhost:5001/api/v0/cat?arg={cid}");
        let response = reqwest::get(&url)
            .await
            .context("local IPFS node request failed")?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}

impl Default for IpfsLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loader_defaults() {
        let loader = IpfsLoader::new();
        assert!(loader.gateway.contains("ipfs.io"));
        assert_eq!(loader.timeout, std::time::Duration::from_secs(30));
    }

    #[test]
    fn custom_gateway() {
        let loader = IpfsLoader::new().with_gateway("https://gateway.pinata.cloud/ipfs");
        assert!(loader.gateway.contains("pinata"));
    }

    #[test]
    fn custom_timeout() {
        let loader = IpfsLoader::new().with_timeout(std::time::Duration::from_secs(60));
        assert_eq!(loader.timeout, std::time::Duration::from_secs(60));
    }
}
