use ethabi::ParamType;

use super::abi;
use crate::error::{Result, WalletError};
use crate::rpc::ethereum::EthereumProvider;

/// Interact with a Uniswap V2–style Router contract.
///
/// Works with any Uniswap V2 fork (SushiSwap, PancakeSwap, etc.) since
/// they all share the same Router02 interface.
pub struct UniswapV2Router {
    router_address: String,
    provider: EthereumProvider,
}

impl UniswapV2Router {
    pub fn new(router_address: &str, provider: EthereumProvider) -> Self {
        Self {
            router_address: router_address.to_string(),
            provider,
        }
    }

    /// Return the router contract address.
    pub fn address(&self) -> &str {
        &self.router_address
    }

    /// Query `factory()` — returns the address of the paired factory contract.
    pub async fn factory(&self) -> Result<String> {
        let data = abi::encode_call_hex("factory()", &[])?;
        let raw = self.provider.call(&self.router_address, &data).await?;
        let tokens = abi::decode_output_hex(&[ParamType::Address], &raw)?;
        tokens
            .into_iter()
            .next()
            .and_then(|t| t.into_address())
            .map(|a| format!("0x{}", hex::encode(a.as_bytes())))
            .ok_or_else(|| WalletError::ContractError("factory() returned no data".into()))
    }

    /// Query `WETH()` — the wrapped native token address used by this router.
    pub async fn weth(&self) -> Result<String> {
        let data = abi::encode_call_hex("WETH()", &[])?;
        let raw = self.provider.call(&self.router_address, &data).await?;
        let tokens = abi::decode_output_hex(&[ParamType::Address], &raw)?;
        tokens
            .into_iter()
            .next()
            .and_then(|t| t.into_address())
            .map(|a| format!("0x{}", hex::encode(a.as_bytes())))
            .ok_or_else(|| WalletError::ContractError("WETH() returned no data".into()))
    }

    /// Query `getAmountsOut(amountIn, path)` — get the expected output amounts
    /// for a given input amount through a swap path.
    ///
    /// `path` is a list of token addresses (e.g. `[WETH, USDC]` for a
    /// WETH→USDC swap).
    pub async fn get_amounts_out(
        &self,
        amount_in: &str,
        path: &[&str],
    ) -> Result<Vec<String>> {
        let amount_token = abi::hex_to_uint256(amount_in)?;
        let path_tokens: Result<Vec<_>> = path.iter().map(|a| abi::hex_to_address(a)).collect();
        let path_tokens = path_tokens?;

        let data = abi::encode_call_hex(
            "getAmountsOut(uint256,address[])",
            &[amount_token, ethabi::Token::Array(path_tokens)],
        )?;

        let raw = self.provider.call(&self.router_address, &data).await?;
        let tokens = abi::decode_output_hex(
            &[ParamType::Array(Box::new(ParamType::Uint(256)))],
            &raw,
        )?;

        match tokens.into_iter().next() {
            Some(ethabi::Token::Array(amounts)) => Ok(amounts
                .into_iter()
                .filter_map(|t| t.into_uint().map(|v| v.to_string()))
                .collect()),
            _ => Err(WalletError::ContractError(
                "getAmountsOut() returned unexpected data".into(),
            )),
        }
    }

    /// Build calldata for `swapExactTokensForTokens(amountIn, amountOutMin, path, to, deadline)`.
    ///
    /// Returns `0x`-prefixed hex. The caller signs and broadcasts the tx.
    pub fn encode_swap_exact_tokens(
        &self,
        amount_in: &str,
        amount_out_min: &str,
        path: &[&str],
        to: &str,
        deadline: &str,
    ) -> Result<String> {
        let amount_in_t = abi::hex_to_uint256(amount_in)?;
        let amount_out_min_t = abi::hex_to_uint256(amount_out_min)?;
        let path_t: Result<Vec<_>> = path.iter().map(|a| abi::hex_to_address(a)).collect();
        let to_t = abi::hex_to_address(to)?;
        let deadline_t = abi::hex_to_uint256(deadline)?;

        abi::encode_call_hex(
            "swapExactTokensForTokens(uint256,uint256,address[],address,uint256)",
            &[
                amount_in_t,
                amount_out_min_t,
                ethabi::Token::Array(path_t?),
                to_t,
                deadline_t,
            ],
        )
    }

    /// Build calldata for `addLiquidity(tokenA, tokenB, amountADesired, amountBDesired,
    /// amountAMin, amountBMin, to, deadline)`.
    #[allow(clippy::too_many_arguments)]
    pub fn encode_add_liquidity(
        &self,
        token_a: &str,
        token_b: &str,
        amount_a_desired: &str,
        amount_b_desired: &str,
        amount_a_min: &str,
        amount_b_min: &str,
        to: &str,
        deadline: &str,
    ) -> Result<String> {
        abi::encode_call_hex(
            "addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)",
            &[
                abi::hex_to_address(token_a)?,
                abi::hex_to_address(token_b)?,
                abi::hex_to_uint256(amount_a_desired)?,
                abi::hex_to_uint256(amount_b_desired)?,
                abi::hex_to_uint256(amount_a_min)?,
                abi::hex_to_uint256(amount_b_min)?,
                abi::hex_to_address(to)?,
                abi::hex_to_uint256(deadline)?,
            ],
        )
    }
}

/// Interact with a Uniswap V2–style Pair (liquidity pool) contract.
pub struct UniswapV2Pair {
    pair_address: String,
    provider: EthereumProvider,
}

impl UniswapV2Pair {
    pub fn new(pair_address: &str, provider: EthereumProvider) -> Self {
        Self {
            pair_address: pair_address.to_string(),
            provider,
        }
    }

    /// Query `getReserves()` — returns `(reserve0, reserve1, blockTimestampLast)`.
    pub async fn get_reserves(&self) -> Result<(String, String, u32)> {
        let data = abi::encode_call_hex("getReserves()", &[])?;
        let raw = self.provider.call(&self.pair_address, &data).await?;
        let tokens = abi::decode_output_hex(
            &[
                ParamType::Uint(112),
                ParamType::Uint(112),
                ParamType::Uint(32),
            ],
            &raw,
        )?;

        if tokens.len() < 3 {
            return Err(WalletError::ContractError(
                "getReserves() returned incomplete data".into(),
            ));
        }
        let r0 = tokens[0]
            .clone()
            .into_uint()
            .map(|v| v.to_string())
            .unwrap_or_default();
        let r1 = tokens[1]
            .clone()
            .into_uint()
            .map(|v| v.to_string())
            .unwrap_or_default();
        let ts = tokens[2]
            .clone()
            .into_uint()
            .map(|v| v.as_u32())
            .unwrap_or(0);
        Ok((r0, r1, ts))
    }

    /// Query `token0()`.
    pub async fn token0(&self) -> Result<String> {
        let data = abi::encode_call_hex("token0()", &[])?;
        let raw = self.provider.call(&self.pair_address, &data).await?;
        let tokens = abi::decode_output_hex(&[ParamType::Address], &raw)?;
        tokens
            .into_iter()
            .next()
            .and_then(|t| t.into_address())
            .map(|a| format!("0x{}", hex::encode(a.as_bytes())))
            .ok_or_else(|| WalletError::ContractError("token0() returned no data".into()))
    }

    /// Query `token1()`.
    pub async fn token1(&self) -> Result<String> {
        let data = abi::encode_call_hex("token1()", &[])?;
        let raw = self.provider.call(&self.pair_address, &data).await?;
        let tokens = abi::decode_output_hex(&[ParamType::Address], &raw)?;
        tokens
            .into_iter()
            .next()
            .and_then(|t| t.into_address())
            .map(|a| format!("0x{}", hex::encode(a.as_bytes())))
            .ok_or_else(|| WalletError::ContractError("token1() returned no data".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_swap_calldata() {
        let provider = EthereumProvider::new("http://localhost:8545");
        let router =
            UniswapV2Router::new("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D", provider);

        let data = router
            .encode_swap_exact_tokens(
                "0xDE0B6B3A7640000", // 1 ETH in wei
                "0x1",               // min 1 wei out
                &[
                    "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
                    "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
                ],
                "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
                "0xFFFFFFFF", // far-future deadline
            )
            .unwrap();

        // swapExactTokensForTokens selector: 0x38ed1739
        assert!(data.starts_with("0x38ed1739"));
    }

    #[test]
    fn encode_add_liquidity_calldata() {
        let provider = EthereumProvider::new("http://localhost:8545");
        let router =
            UniswapV2Router::new("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D", provider);

        let data = router
            .encode_add_liquidity(
                "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
                "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
                "0xDE0B6B3A7640000",
                "0x3B9ACA00",
                "0x1",
                "0x1",
                "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
                "0xFFFFFFFF",
            )
            .unwrap();

        // addLiquidity selector: 0xe8e33700
        assert!(data.starts_with("0xe8e33700"));
    }
}
