use getrandom::getrandom;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum SessionMode {
    #[default]
    Disconnected,
    BrowserWallet,
    ImportedAddress,
    PreviewWallet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WalletSession {
    pub mode: SessionMode,
    pub wallet_name: String,
    pub address: Option<String>,
    pub status_message: String,
}

impl WalletSession {
    pub fn disconnected() -> Self {
        Self {
            mode: SessionMode::Disconnected,
            wallet_name: "No wallet connected".to_string(),
            address: None,
            status_message: "Connect a browser wallet or import an address.".to_string(),
        }
    }

    pub fn preview() -> Self {
        let address = generate_preview_address();
        Self {
            mode: SessionMode::PreviewWallet,
            wallet_name: "Preview Wallet".to_string(),
            address: Some(address),
            status_message: "Local preview wallet created in this browser.".to_string(),
        }
    }

    pub fn imported(address: String) -> Self {
        Self {
            mode: SessionMode::ImportedAddress,
            wallet_name: "Imported Address".to_string(),
            address: Some(normalize_address(&address)),
            status_message: "Watch-only address imported into this browser.".to_string(),
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub fn browser(address: String) -> Self {
        Self {
            mode: SessionMode::BrowserWallet,
            wallet_name: "Browser Wallet".to_string(),
            address: Some(normalize_address(&address)),
            status_message: "Browser wallet connected.".to_string(),
        }
    }

    pub fn short_address(&self) -> String {
        self.address
            .as_deref()
            .map(shorten_address)
            .unwrap_or_else(|| "Not connected".to_string())
    }
}

pub fn normalize_address(address: &str) -> String {
    let trimmed = address.trim();
    if trimmed.starts_with("0x") {
        trimmed.to_string()
    } else {
        format!("0x{trimmed}")
    }
}

pub fn shorten_address(address: &str) -> String {
    let normalized = normalize_address(address);
    if normalized.len() <= 12 {
        return normalized;
    }
    format!("{}...{}", &normalized[..8], &normalized[normalized.len() - 4..])
}

pub fn is_valid_evm_address(address: &str) -> bool {
    let normalized = normalize_address(address);
    normalized.len() == 42
        && normalized.starts_with("0x")
        && normalized.chars().skip(2).all(|c| c.is_ascii_hexdigit())
}

pub fn load_session() -> Option<WalletSession> {
    #[cfg(target_arch = "wasm32")]
    {
        let storage = web_sys::window()
            .and_then(|window| window.local_storage().ok().flatten())?;
        let raw = storage.get_item("xorion.webapp.session").ok().flatten()?;
        let session: WalletSession = serde_json::from_str(&raw).ok()?;

        match session.address.as_deref() {
            Some(address) if !is_valid_evm_address(address) => None,
            _ => Some(session),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

pub fn save_session(session: &WalletSession) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let storage = web_sys::window()
            .and_then(|window| window.local_storage().ok().flatten())
            .ok_or_else(|| "Browser storage is unavailable".to_string())?;
        let payload = serde_json::to_string(session)
            .map_err(|err| format!("Could not serialize wallet session: {err}"))?;
        storage
            .set_item("xorion.webapp.session", &payload)
            .map_err(|_| "Could not save wallet session in browser storage".to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = session;
        Ok(())
    }
}

pub fn clear_session() -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let storage = web_sys::window()
            .and_then(|window| window.local_storage().ok().flatten())
            .ok_or_else(|| "Browser storage is unavailable".to_string())?;
        storage
            .remove_item("xorion.webapp.session")
            .map_err(|_| "Could not clear wallet session".to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        Ok(())
    }
}

pub fn generate_preview_address() -> String {
    let mut bytes = [0_u8; 20];
    if getrandom(&mut bytes).is_err() {
        bytes = [0x11; 20];
    }
    let hex = bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    format!("0x{hex}")
}

#[cfg(target_arch = "wasm32")]
pub async fn connect_browser_wallet() -> Result<WalletSession, String> {
    use js_sys::{Array, Function, Object, Promise, Reflect};
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;
    let ethereum = Reflect::get(window.as_ref(), &JsValue::from_str("ethereum"))
        .map_err(|_| "Browser wallet provider is unavailable".to_string())?;

    if ethereum.is_undefined() || ethereum.is_null() {
        return Err("No browser wallet detected. Install MetaMask or another injected wallet.".to_string());
    }

    let request = Reflect::get(&ethereum, &JsValue::from_str("request"))
        .map_err(|_| "Browser wallet request API is unavailable".to_string())?
        .dyn_into::<Function>()
        .map_err(|_| "Browser wallet request API is invalid".to_string())?;

    let request_payload = Object::new();
    Reflect::set(
        &request_payload,
        &JsValue::from_str("method"),
        &JsValue::from_str("eth_requestAccounts"),
    )
    .map_err(|_| "Could not prepare wallet request".to_string())?;

    let promise = request
        .call1(&ethereum, &request_payload)
        .map_err(|_| "Wallet connection request was rejected".to_string())?
        .dyn_into::<Promise>()
        .map_err(|_| "Wallet response was invalid".to_string())?;

    let accounts = JsFuture::from(promise)
        .await
        .map_err(|_| "Wallet connection request failed".to_string())?;

    let array = Array::from(&accounts);
    let address = array
        .get(0)
        .as_string()
        .ok_or_else(|| "No account was returned by the browser wallet".to_string())?;

    Ok(WalletSession::browser(address))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn connect_browser_wallet() -> Result<WalletSession, String> {
    Err("Browser wallet connection is only available in the wasm build".to_string())
}
