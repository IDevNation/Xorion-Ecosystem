//! Kernel-integration client API for Xorion Web3 OS.
//!
//! This module provides syscall-style functions that applications use to
//! interact with the wallet scheme daemon. On Redox OS, these communicate
//! via the `wallet:` scheme. On Linux/macOS, they talk to the Unix socket
//! development daemon.
//!
//! # Usage
//!
//! ```no_run
//! use xorion_sdk::kernel::WalletClient;
//!
//! let mut client = WalletClient::connect().unwrap();
//! client.wallet_init("abandon abandon ... about").unwrap();
//!
//! let mut buf = [0u8; 42];
//! let len = client.wallet_eth_address(&mut buf).unwrap();
//! let address = std::str::from_utf8(&buf[..len]).unwrap();
//! ```

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;

use crate::error::{Result, WalletError};

/// Socket path — matches the scheme daemon.
const SOCKET_PATH: &str = "/tmp/xorion-wallet.sock";

/// Chain identifiers matching the kernel ABI.
pub const CHAIN_ETHEREUM: u32 = 0;
pub const CHAIN_SOLANA: u32 = 1;

/// Client handle for communicating with the wallet scheme daemon.
///
/// Mirrors the syscall interface requested in Phase 4 but communicates
/// over IPC (Unix socket / Redox scheme) instead of raw syscalls.
pub struct WalletClient {
    reader: BufReader<UnixStream>,
    writer: UnixStream,
}

impl WalletClient {
    /// Connect to the wallet scheme daemon.
    ///
    /// On Redox OS this opens `wallet:` via the scheme.
    /// On Linux/macOS it connects to the development Unix socket.
    pub fn connect() -> Result<Self> {
        let stream = UnixStream::connect(SOCKET_PATH)
            .map_err(|e| WalletError::NetworkError(format!("cannot connect to wallet daemon: {e}")))?;
        let reader = BufReader::new(stream.try_clone().map_err(|e| {
            WalletError::NetworkError(format!("stream clone failed: {e}"))
        })?);
        Ok(Self {
            reader,
            writer: stream,
        })
    }

    /// Initialize the wallet from a mnemonic phrase.
    pub fn wallet_init(&mut self, mnemonic: &str) -> Result<()> {
        let req = format!(r#"{{"cmd":"init_wallet","mnemonic":"{mnemonic}"}}"#);
        let resp = self.send_request(&req)?;
        self.check_response(&resp)
    }

    /// Get the Ethereum address for the current wallet session.
    ///
    /// Writes the address (e.g. `"0x9858..."`) into `buf` and returns the
    /// number of bytes written — matching the syscall signature:
    ///
    /// ```text
    /// fn wallet_eth_address(buf: *mut u8, len: usize) -> Result<usize>
    /// ```
    pub fn wallet_eth_address(&mut self, buf: &mut [u8]) -> Result<usize> {
        let resp = self.send_request(r#"{"cmd":"eth_address"}"#)?;
        self.extract_data_into_buf(&resp, buf)
    }

    /// Get the Solana address for the current wallet session.
    ///
    /// ```text
    /// fn wallet_solana_address(buf: *mut u8, len: usize) -> Result<usize>
    /// ```
    pub fn wallet_solana_address(&mut self, buf: &mut [u8]) -> Result<usize> {
        let resp = self.send_request(r#"{"cmd":"solana_address"}"#)?;
        self.extract_data_into_buf(&resp, buf)
    }

    /// Sign a transaction and write the signature into `sig_out`.
    ///
    /// ```text
    /// fn wallet_sign_transaction(chain: u32, tx_data: *const u8, tx_len: usize,
    ///                            sig_out: *mut u8, sig_max: usize) -> Result<usize>
    /// ```
    pub fn wallet_sign_transaction(
        &mut self,
        chain: u32,
        tx_data: &[u8],
        sig_out: &mut [u8],
    ) -> Result<usize> {
        let tx_hex = hex::encode(tx_data);
        let req = format!(r#"{{"cmd":"sign_transaction","chain":{chain},"tx_data":"{tx_hex}"}}"#);
        let resp = self.send_request(&req)?;
        self.extract_data_into_buf(&resp, sig_out)
    }

    /// Query the balance for an address on the given chain.
    ///
    /// ```text
    /// fn wallet_get_balance(chain: u32, address: *const u8, addr_len: usize,
    ///                       balance_out: *mut u8) -> Result<u64>
    /// ```
    pub fn wallet_get_balance(
        &mut self,
        chain: u32,
        address: &str,
        balance_out: &mut [u8],
    ) -> Result<usize> {
        let req =
            format!(r#"{{"cmd":"get_balance","chain":{chain},"address":"{address}"}}"#);
        let resp = self.send_request(&req)?;
        self.extract_data_into_buf(&resp, balance_out)
    }

    /// Query daemon status.
    pub fn wallet_status(&mut self) -> Result<String> {
        let resp = self.send_request(r#"{"cmd":"status"}"#)?;
        self.extract_data(&resp)
    }

    // ── internal helpers ────────────────────────────────────────

    fn send_request(&mut self, json_line: &str) -> Result<String> {
        writeln!(self.writer, "{json_line}")
            .map_err(|e| WalletError::NetworkError(format!("write failed: {e}")))?;
        self.writer
            .flush()
            .map_err(|e| WalletError::NetworkError(format!("flush failed: {e}")))?;

        let mut line = String::new();
        self.reader
            .read_line(&mut line)
            .map_err(|e| WalletError::NetworkError(format!("read failed: {e}")))?;
        Ok(line)
    }

    fn extract_data(&self, resp_json: &str) -> Result<String> {
        let v: serde_json::Value = serde_json::from_str(resp_json)?;
        match v.get("status").and_then(|s| s.as_str()) {
            Some("ok") => v
                .get("data")
                .and_then(|d| d.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| WalletError::InvalidResponse("missing data field".into())),
            Some("error") => {
                let msg = v
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("unknown error");
                Err(WalletError::ContractError(msg.to_string()))
            }
            _ => Err(WalletError::InvalidResponse(
                "unknown response format".into(),
            )),
        }
    }

    fn extract_data_into_buf(&self, resp_json: &str, buf: &mut [u8]) -> Result<usize> {
        let data = self.extract_data(resp_json)?;
        let bytes = data.as_bytes();
        let copy_len = bytes.len().min(buf.len());
        buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
        Ok(copy_len)
    }

    fn check_response(&self, resp_json: &str) -> Result<()> {
        self.extract_data(resp_json)?;
        Ok(())
    }
}
