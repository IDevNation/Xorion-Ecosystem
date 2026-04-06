use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RpcOverview {
    pub chain_id: String,
    pub chain_name: String,
    pub latest_block: String,
    pub balance_wei: String,
    pub balance_display: String,
    pub rpc_status: String,
    pub network_health: String,
    pub synced_at: String,
}

#[derive(Deserialize)]
struct RpcResponse {
    result: Option<Value>,
    error: Option<RpcError>,
}

#[derive(Deserialize)]
struct RpcError {
    message: String,
}

pub async fn fetch_overview(rpc_url: &str, address: Option<&str>) -> Result<RpcOverview, String> {
    let chain_id_hex = rpc_call(rpc_url, "eth_chainId", json!([])).await?;
    let block_hex = rpc_call(rpc_url, "eth_blockNumber", json!([])).await?;
    let chain_id = hex_to_u64(&chain_id_hex)?;
    let latest_block = hex_to_u64(&block_hex)?;

    let (balance_wei, balance_display) = if let Some(account) = address {
        let balance_hex = rpc_call(rpc_url, "eth_getBalance", json!([account, "latest"])).await?;
        let wei = hex_to_u128(&balance_hex)?;
        (wei.to_string(), format_native_balance(wei))
    } else {
        ("0".to_string(), "0.0000 ETH".to_string())
    };

    Ok(RpcOverview {
        chain_id: format!("0x{chain_id:x}"),
        chain_name: chain_name(chain_id).to_string(),
        latest_block: latest_block.to_string(),
        balance_wei,
        balance_display,
        rpc_status: "Connected".to_string(),
        network_health: "Healthy".to_string(),
        synced_at: "Live RPC".to_string(),
    })
}

async fn rpc_call(rpc_url: &str, method: &str, params: Value) -> Result<String, String> {
    let response = Request::post(rpc_url)
        .header("content-type", "application/json")
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        }))
        .map_err(|err| format!("Could not create RPC request: {err}"))?
        .send()
        .await
        .map_err(|err| format!("RPC request failed: {err}. Check the endpoint URL and browser network access."))?;

    if !response.ok() {
        return Err(format!(
            "RPC endpoint returned HTTP {}. Use an EVM-compatible JSON-RPC URL.",
            response.status()
        ));
    }

    let payload: RpcResponse = response
        .json()
        .await
        .map_err(|err| format!("Could not parse RPC response: {err}"))?;

    if let Some(error) = payload.error {
        return Err(format!(
            "RPC error: {}. Use an EVM-compatible endpoint for this MVP.",
            error.message
        ));
    }

    payload
        .result
        .and_then(|value| value.as_str().map(ToOwned::to_owned))
        .ok_or_else(|| format!("RPC method {method} returned no result. Use an EVM-compatible endpoint for this MVP."))
}

fn chain_name(chain_id: u64) -> &'static str {
    match chain_id {
        1 => "Ethereum Mainnet",
        11155111 => "Ethereum Sepolia",
        137 => "Polygon",
        8453 => "Base",
        _ => "Custom EVM",
    }
}

fn hex_to_u64(value: &str) -> Result<u64, String> {
    let normalized = value.trim_start_matches("0x");
    u64::from_str_radix(normalized, 16)
        .map_err(|err| format!("Could not parse rpc value {value}: {err}"))
}

fn hex_to_u128(value: &str) -> Result<u128, String> {
    let normalized = value.trim_start_matches("0x");
    u128::from_str_radix(normalized, 16)
        .map_err(|err| format!("Could not parse rpc value {value}: {err}"))
}

fn format_native_balance(wei: u128) -> String {
    let whole = wei / 1_000_000_000_000_000_000;
    let fractional = (wei % 1_000_000_000_000_000_000) / 100_000_000_000_000;
    format!("{whole}.{fractional:04} ETH")
}
