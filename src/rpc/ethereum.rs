use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use super::types::{Chain, JsonRpcRequest, JsonRpcResponse, TransactionReceipt};
use super::ChainProvider;
use crate::error::{Result, WalletError};

/// Ethereum JSON-RPC provider.
///
/// Talks to any Ethereum-compatible node (mainnet, Sepolia, Hardhat, Anvil, etc.).
pub struct EthereumProvider {
    endpoint: String,
    client: Client,
}

impl EthereumProvider {
    /// Create a provider pointing at the given JSON-RPC URL.
    ///
    /// # Examples
    /// ```no_run
    /// use xorion_sdk::rpc::ethereum::EthereumProvider;
    /// let provider = EthereumProvider::new("https://eth.llamarpc.com");
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

    /// Get the transaction count (nonce) for an address.
    pub async fn get_transaction_count(&self, address: &str) -> Result<u64> {
        let resp = self
            .rpc_call("eth_getTransactionCount", json!([address, "latest"]))
            .await?;
        let hex_str = resp
            .result
            .as_ref()
            .and_then(|v| v.as_str())
            .ok_or_else(|| WalletError::InvalidResponse("missing nonce".into()))?;
        parse_hex_u64(hex_str)
    }

    /// Execute a read-only `eth_call` against a contract.
    ///
    /// `to` is the contract address, `data` is the ABI-encoded calldata
    /// (with `0x` prefix). Returns the raw hex-encoded return data.
    pub async fn call(&self, to: &str, data: &str) -> Result<String> {
        let resp = self
            .rpc_call(
                "eth_call",
                json!([{"to": to, "data": data}, "latest"]),
            )
            .await?;
        resp.result
            .as_ref()
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WalletError::InvalidResponse("eth_call returned no data".into()))
    }

    /// Get the current chain ID.
    pub async fn get_chain_id(&self) -> Result<u64> {
        let resp = self.rpc_call("eth_chainId", json!([])).await?;
        let hex_str = resp
            .result
            .as_ref()
            .and_then(|v| v.as_str())
            .ok_or_else(|| WalletError::InvalidResponse("missing chain ID".into()))?;
        parse_hex_u64(hex_str)
    }
}

#[async_trait]
impl ChainProvider for EthereumProvider {
    fn chain(&self) -> Chain {
        Chain::Ethereum
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    async fn get_block_number(&self) -> Result<u64> {
        let resp = self.rpc_call("eth_blockNumber", json!([])).await?;
        let hex_str = resp
            .result
            .as_ref()
            .and_then(|v| v.as_str())
            .ok_or_else(|| WalletError::InvalidResponse("missing block number".into()))?;
        parse_hex_u64(hex_str)
    }

    async fn get_balance(&self, address: &str) -> Result<String> {
        let resp = self
            .rpc_call("eth_getBalance", json!([address, "latest"]))
            .await?;
        let hex_str = resp
            .result
            .as_ref()
            .and_then(|v| v.as_str())
            .ok_or_else(|| WalletError::InvalidResponse("missing balance".into()))?;
        Ok(hex_str.to_string())
    }

    async fn send_raw_transaction(&self, signed_tx_hex: &str) -> Result<TransactionReceipt> {
        let tx = if signed_tx_hex.starts_with("0x") {
            signed_tx_hex.to_string()
        } else {
            format!("0x{signed_tx_hex}")
        };

        let resp = self
            .rpc_call("eth_sendRawTransaction", json!([tx]))
            .await?;
        let tx_hash = resp
            .result
            .as_ref()
            .and_then(|v| v.as_str())
            .ok_or_else(|| WalletError::TransactionError("no tx hash returned".into()))?
            .to_string();

        Ok(TransactionReceipt {
            chain: Chain::Ethereum,
            tx_hash,
        })
    }

    async fn get_fee_estimate(&self) -> Result<String> {
        let resp = self.rpc_call("eth_gasPrice", json!([])).await?;
        let hex_str = resp
            .result
            .as_ref()
            .and_then(|v| v.as_str())
            .ok_or_else(|| WalletError::InvalidResponse("missing gas price".into()))?;
        Ok(hex_str.to_string())
    }
}

/// Parse an Ethereum-style `"0x..."` hex string into a `u64`.
fn parse_hex_u64(hex_str: &str) -> Result<u64> {
    let stripped = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    u64::from_str_radix(stripped, 16)
        .map_err(|e| WalletError::InvalidResponse(format!("bad hex value \"{hex_str}\": {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex_values() {
        assert_eq!(parse_hex_u64("0x1").unwrap(), 1);
        assert_eq!(parse_hex_u64("0xff").unwrap(), 255);
        assert_eq!(parse_hex_u64("0x0").unwrap(), 0);
        assert_eq!(parse_hex_u64("0x10").unwrap(), 16);
    }

    #[test]
    fn provider_has_correct_chain() {
        let p = EthereumProvider::new("http://localhost:8545");
        assert_eq!(p.chain(), Chain::Ethereum);
        assert_eq!(p.endpoint(), "http://localhost:8545");
    }
}
