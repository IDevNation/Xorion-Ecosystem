use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use super::types::{Chain, JsonRpcRequest, JsonRpcResponse, TransactionReceipt};
use super::ChainProvider;
use crate::error::{Result, WalletError};

/// Solana JSON-RPC provider.
///
/// Talks to any Solana-compatible node (mainnet-beta, devnet, testnet, local validator).
pub struct SolanaProvider {
    endpoint: String,
    client: Client,
}

impl SolanaProvider {
    /// Create a provider pointing at the given JSON-RPC URL.
    ///
    /// # Examples
    /// ```no_run
    /// use xorion_sdk::rpc::solana::SolanaProvider;
    /// let provider = SolanaProvider::new("https://api.devnet.solana.com");
    /// ```
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            client: Client::new(),
        }
    }

    /// Send a JSON-RPC request and return the parsed response.
    async fn rpc_call(&self, method: &str, params: serde_json::Value) -> Result<JsonRpcResponse> {
        let req = JsonRpcRequest::new(method, params);
        let resp = self
            .client
            .post(&self.endpoint)
            .json(&req)
            .send()
            .await?
            .json::<JsonRpcResponse>()
            .await?;

        if let Some(err) = &resp.error {
            return Err(WalletError::RpcError(format!(
                "code {}: {}",
                err.code, err.message
            )));
        }
        Ok(resp)
    }

    /// Get account info as raw JSON for the given public key.
    pub async fn get_account_info(&self, pubkey: &str) -> Result<serde_json::Value> {
        let resp = self
            .rpc_call(
                "getAccountInfo",
                json!([pubkey, {"encoding": "base64"}]),
            )
            .await?;
        resp.result
            .ok_or_else(|| WalletError::InvalidResponse("missing account info".into()))
    }

    /// Get the recent blockhash (needed to construct transactions).
    pub async fn get_latest_blockhash(&self) -> Result<String> {
        let resp = self.rpc_call("getLatestBlockhash", json!([])).await?;
        let hash = resp
            .result
            .as_ref()
            .and_then(|v| v.get("value"))
            .and_then(|v| v.get("blockhash"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| WalletError::InvalidResponse("missing blockhash".into()))?;
        Ok(hash.to_string())
    }
}

#[async_trait]
impl ChainProvider for SolanaProvider {
    fn chain(&self) -> Chain {
        Chain::Solana
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    async fn get_block_number(&self) -> Result<u64> {
        let resp = self.rpc_call("getSlot", json!([])).await?;
        resp.result
            .as_ref()
            .and_then(|v| v.as_u64())
            .ok_or_else(|| WalletError::InvalidResponse("missing slot number".into()))
    }

    async fn get_balance(&self, address: &str) -> Result<String> {
        let resp = self.rpc_call("getBalance", json!([address])).await?;
        let lamports = resp
            .result
            .as_ref()
            .and_then(|v| v.get("value"))
            .and_then(|v| v.as_u64())
            .ok_or_else(|| WalletError::InvalidResponse("missing balance".into()))?;
        Ok(lamports.to_string())
    }

    async fn send_raw_transaction(&self, signed_tx_hex: &str) -> Result<TransactionReceipt> {
        let resp = self
            .rpc_call(
                "sendTransaction",
                json!([signed_tx_hex, {"encoding": "base64"}]),
            )
            .await?;
        let tx_hash = resp
            .result
            .as_ref()
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                WalletError::TransactionError("no signature returned".into())
            })?
            .to_string();

        Ok(TransactionReceipt {
            chain: Chain::Solana,
            tx_hash,
        })
    }

    async fn get_fee_estimate(&self) -> Result<String> {
        self.get_latest_blockhash().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_has_correct_chain() {
        let p = SolanaProvider::new("https://api.devnet.solana.com");
        assert_eq!(p.chain(), Chain::Solana);
        assert_eq!(p.endpoint(), "https://api.devnet.solana.com");
    }
}
