use ethabi::ParamType;

use super::abi;
use crate::error::{Result, WalletError};
use crate::rpc::ethereum::EthereumProvider;

/// High-level interface for interacting with any ERC-20 token contract.
pub struct Erc20 {
    contract_address: String,
    provider: EthereumProvider,
}

impl Erc20 {
    /// Create a new ERC-20 handle for the token at `contract_address`.
    pub fn new(contract_address: &str, provider: EthereumProvider) -> Self {
        Self {
            contract_address: contract_address.to_string(),
            provider,
        }
    }

    /// Return the contract address.
    pub fn address(&self) -> &str {
        &self.contract_address
    }

    /// Query `name()` — the human-readable token name.
    pub async fn name(&self) -> Result<String> {
        let data = abi::encode_call_hex("name()", &[])?;
        let raw = self.eth_call(&data).await?;
        let tokens = abi::decode_output_hex(&[ParamType::String], &raw)?;
        tokens
            .into_iter()
            .next()
            .and_then(|t| t.into_string())
            .ok_or_else(|| WalletError::ContractError("name() returned no data".into()))
    }

    /// Query `symbol()` — the token ticker (e.g. "USDC").
    pub async fn symbol(&self) -> Result<String> {
        let data = abi::encode_call_hex("symbol()", &[])?;
        let raw = self.eth_call(&data).await?;
        let tokens = abi::decode_output_hex(&[ParamType::String], &raw)?;
        tokens
            .into_iter()
            .next()
            .and_then(|t| t.into_string())
            .ok_or_else(|| WalletError::ContractError("symbol() returned no data".into()))
    }

    /// Query `decimals()` — number of decimals the token uses.
    pub async fn decimals(&self) -> Result<u8> {
        let data = abi::encode_call_hex("decimals()", &[])?;
        let raw = self.eth_call(&data).await?;
        let tokens = abi::decode_output_hex(&[ParamType::Uint(8)], &raw)?;
        tokens
            .into_iter()
            .next()
            .and_then(|t| t.into_uint())
            .map(|v| v.as_u64() as u8)
            .ok_or_else(|| WalletError::ContractError("decimals() returned no data".into()))
    }

    /// Query `totalSupply()` — total token supply in the smallest unit.
    pub async fn total_supply(&self) -> Result<String> {
        let data = abi::encode_call_hex("totalSupply()", &[])?;
        let raw = self.eth_call(&data).await?;
        let tokens = abi::decode_output_hex(&[ParamType::Uint(256)], &raw)?;
        tokens
            .into_iter()
            .next()
            .and_then(|t| t.into_uint())
            .map(|v| v.to_string())
            .ok_or_else(|| WalletError::ContractError("totalSupply() returned no data".into()))
    }

    /// Query `balanceOf(address)` — token balance for the given address.
    pub async fn balance_of(&self, owner: &str) -> Result<String> {
        let addr = abi::hex_to_address(owner)?;
        let data = abi::encode_call_hex("balanceOf(address)", &[addr])?;
        let raw = self.eth_call(&data).await?;
        let tokens = abi::decode_output_hex(&[ParamType::Uint(256)], &raw)?;
        tokens
            .into_iter()
            .next()
            .and_then(|t| t.into_uint())
            .map(|v| v.to_string())
            .ok_or_else(|| WalletError::ContractError("balanceOf() returned no data".into()))
    }

    /// Query `allowance(owner, spender)` — how many tokens `spender` may
    /// transfer on behalf of `owner`.
    pub async fn allowance(&self, owner: &str, spender: &str) -> Result<String> {
        let owner_token = abi::hex_to_address(owner)?;
        let spender_token = abi::hex_to_address(spender)?;
        let data = abi::encode_call_hex(
            "allowance(address,address)",
            &[owner_token, spender_token],
        )?;
        let raw = self.eth_call(&data).await?;
        let tokens = abi::decode_output_hex(&[ParamType::Uint(256)], &raw)?;
        tokens
            .into_iter()
            .next()
            .and_then(|t| t.into_uint())
            .map(|v| v.to_string())
            .ok_or_else(|| WalletError::ContractError("allowance() returned no data".into()))
    }

    /// Build the calldata for `transfer(address,uint256)`.
    ///
    /// Returns the `0x`-prefixed hex calldata. The caller is responsible for
    /// signing and broadcasting the transaction via the provider.
    pub fn encode_transfer(&self, to: &str, amount: &str) -> Result<String> {
        let to_token = abi::hex_to_address(to)?;
        let amount_token = abi::hex_to_uint256(amount)?;
        abi::encode_call_hex("transfer(address,uint256)", &[to_token, amount_token])
    }

    /// Build the calldata for `approve(address,uint256)`.
    pub fn encode_approve(&self, spender: &str, amount: &str) -> Result<String> {
        let spender_token = abi::hex_to_address(spender)?;
        let amount_token = abi::hex_to_uint256(amount)?;
        abi::encode_call_hex("approve(address,uint256)", &[spender_token, amount_token])
    }

    /// Build the calldata for `transferFrom(address,address,uint256)`.
    pub fn encode_transfer_from(
        &self,
        from: &str,
        to: &str,
        amount: &str,
    ) -> Result<String> {
        let from_token = abi::hex_to_address(from)?;
        let to_token = abi::hex_to_address(to)?;
        let amount_token = abi::hex_to_uint256(amount)?;
        abi::encode_call_hex(
            "transferFrom(address,address,uint256)",
            &[from_token, to_token, amount_token],
        )
    }

    /// Low-level `eth_call` to the contract address.
    async fn eth_call(&self, data: &str) -> Result<String> {
        self.provider.call(&self.contract_address, data).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_erc20() -> Erc20 {
        let provider = EthereumProvider::new("http://localhost:8545");
        Erc20::new("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", provider)
    }

    #[test]
    fn encode_transfer_calldata() {
        let erc20 = dummy_erc20();
        let data = erc20
            .encode_transfer(
                "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
                "0x64", // 100 in hex
            )
            .unwrap();
        // Must start with the transfer selector
        assert!(data.starts_with("0xa9059cbb"));
        // 4-byte selector + 2 * 32-byte params = 68 bytes = 136 hex chars + "0x"
        assert_eq!(data.len(), 2 + 136);
    }

    #[test]
    fn encode_approve_calldata() {
        let erc20 = dummy_erc20();
        let data = erc20
            .encode_approve(
                "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
                "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            )
            .unwrap();
        assert!(data.starts_with("0x095ea7b3"));
    }

    #[test]
    fn encode_transfer_from_calldata() {
        let erc20 = dummy_erc20();
        let data = erc20
            .encode_transfer_from(
                "0x0000000000000000000000000000000000000001",
                "0x0000000000000000000000000000000000000002",
                "0x1",
            )
            .unwrap();
        // transferFrom selector: 0x23b872dd
        assert!(data.starts_with("0x23b872dd"));
        // 4 + 3*32 = 100 bytes = 200 hex + "0x"
        assert_eq!(data.len(), 2 + 200);
    }

    #[test]
    fn address_is_stored() {
        let erc20 = dummy_erc20();
        assert_eq!(
            erc20.address(),
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
        );
    }
}
