use ethabi::{Function, Param, ParamType, StateMutability, Token};
use tiny_keccak::{Hasher, Keccak};

use crate::error::{Result, WalletError};

/// Compute the 4-byte function selector from a canonical signature like
/// `"transfer(address,uint256)"`.
pub fn function_selector(signature: &str) -> [u8; 4] {
    let mut keccak = Keccak::v256();
    let mut hash = [0u8; 32];
    keccak.update(signature.as_bytes());
    keccak.finalize(&mut hash);
    let mut selector = [0u8; 4];
    selector.copy_from_slice(&hash[..4]);
    selector
}

/// ABI-encode a function call: 4-byte selector + encoded arguments.
pub fn encode_call(signature: &str, tokens: &[Token]) -> Result<Vec<u8>> {
    let selector = function_selector(signature);
    let encoded_params = ethabi::encode(tokens);

    let mut calldata = Vec::with_capacity(4 + encoded_params.len());
    calldata.extend_from_slice(&selector);
    calldata.extend_from_slice(&encoded_params);
    Ok(calldata)
}

/// ABI-encode a function call and return it as a `0x`-prefixed hex string
/// (ready for `eth_call` / `eth_sendTransaction`).
pub fn encode_call_hex(signature: &str, tokens: &[Token]) -> Result<String> {
    let calldata = encode_call(signature, tokens)?;
    Ok(format!("0x{}", hex::encode(calldata)))
}

/// Decode raw bytes returned by `eth_call` into typed tokens using the given
/// output parameter types.
pub fn decode_output(types: &[ParamType], data: &[u8]) -> Result<Vec<Token>> {
    ethabi::decode(types, data).map_err(|e| WalletError::AbiError(e.to_string()))
}

/// Convenience: decode a `0x`-prefixed hex string returned by an RPC call.
pub fn decode_output_hex(types: &[ParamType], hex_data: &str) -> Result<Vec<Token>> {
    let stripped = hex_data.strip_prefix("0x").unwrap_or(hex_data);
    let bytes =
        hex::decode(stripped).map_err(|e| WalletError::AbiError(format!("bad hex: {e}")))?;
    decode_output(types, &bytes)
}

/// Build an `ethabi::Function` descriptor from a name, input types, and output types.
///
/// This is useful for more complex encoding/decoding scenarios that need the
/// full `Function` struct (e.g. generating full ABI JSON).
pub fn build_function(
    name: &str,
    inputs: Vec<(&str, ParamType)>,
    outputs: Vec<(&str, ParamType)>,
) -> Function {
    Function {
        name: name.to_string(),
        inputs: inputs
            .into_iter()
            .map(|(n, t)| Param {
                name: n.to_string(),
                kind: t,
                internal_type: None,
            })
            .collect(),
        outputs: outputs
            .into_iter()
            .map(|(n, t)| Param {
                name: n.to_string(),
                kind: t,
                internal_type: None,
            })
            .collect(),
        #[allow(deprecated)]
        constant: None,
        state_mutability: StateMutability::NonPayable,
    }
}

/// Parse a hex-encoded 256-bit value (with or without `0x` prefix) into an
/// `ethabi::Token::Uint`.
pub fn hex_to_uint256(hex_str: &str) -> Result<Token> {
    let stripped = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    // Pad to 64 hex chars (32 bytes) if shorter
    let padded = format!("{:0>64}", stripped);
    let bytes =
        hex::decode(&padded).map_err(|e| WalletError::AbiError(format!("bad hex: {e}")))?;
    Ok(Token::Uint(ethabi::ethereum_types::U256::from_big_endian(
        &bytes,
    )))
}

/// Parse a `0x`-prefixed hex address string into an `ethabi::Token::Address`.
pub fn hex_to_address(hex_str: &str) -> Result<Token> {
    let stripped = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    let bytes =
        hex::decode(stripped).map_err(|e| WalletError::AbiError(format!("bad hex: {e}")))?;
    if bytes.len() != 20 {
        return Err(WalletError::AbiError(format!(
            "address must be 20 bytes, got {}",
            bytes.len()
        )));
    }
    Ok(Token::Address(ethabi::ethereum_types::H160::from_slice(
        &bytes,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_transfer() {
        // keccak256("transfer(address,uint256)") starts with 0xa9059cbb
        let sel = function_selector("transfer(address,uint256)");
        assert_eq!(hex::encode(sel), "a9059cbb");
    }

    #[test]
    fn selector_balance_of() {
        // keccak256("balanceOf(address)") starts with 0x70a08231
        let sel = function_selector("balanceOf(address)");
        assert_eq!(hex::encode(sel), "70a08231");
    }

    #[test]
    fn selector_approve() {
        let sel = function_selector("approve(address,uint256)");
        assert_eq!(hex::encode(sel), "095ea7b3");
    }

    #[test]
    fn encode_call_produces_correct_length() {
        let addr = hex_to_address("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045").unwrap();
        let calldata = encode_call("balanceOf(address)", &[addr]).unwrap();
        // 4 selector + 32 address = 36 bytes
        assert_eq!(calldata.len(), 36);
    }

    #[test]
    fn encode_call_hex_has_prefix() {
        let addr = hex_to_address("0x0000000000000000000000000000000000000001").unwrap();
        let hex_data = encode_call_hex("balanceOf(address)", &[addr]).unwrap();
        assert!(hex_data.starts_with("0x70a08231"));
    }

    #[test]
    fn decode_uint256_output() {
        // 32 bytes all zero except last byte = 42
        let mut data = vec![0u8; 32];
        data[31] = 42;
        let tokens = decode_output(&[ParamType::Uint(256)], &data).unwrap();
        assert_eq!(tokens.len(), 1);
        let val = tokens[0].clone().into_uint().unwrap();
        assert_eq!(val.as_u64(), 42);
    }

    #[test]
    fn hex_to_address_validates_length() {
        assert!(hex_to_address("0xaabb").is_err());
        assert!(hex_to_address("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045").is_ok());
    }

    #[test]
    fn hex_to_uint256_parses() {
        let token = hex_to_uint256("0xff").unwrap();
        let val = token.into_uint().unwrap();
        assert_eq!(val.as_u64(), 255);
    }

    #[test]
    fn build_function_descriptor() {
        let f = build_function(
            "transfer",
            vec![("to", ParamType::Address), ("amount", ParamType::Uint(256))],
            vec![("success", ParamType::Bool)],
        );
        assert_eq!(f.name, "transfer");
        assert_eq!(f.inputs.len(), 2);
        assert_eq!(f.outputs.len(), 1);
    }
}
