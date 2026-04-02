#![allow(non_snake_case)]

mod components;

use components::{Dashboard, Receive, Send, Settings};
use dioxus::prelude::*;

/// Global CSS for the dark theme.
const GLOBAL_CSS: &str = r#"
* { margin: 0; padding: 0; box-sizing: border-box; }

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, sans-serif;
    background: #0d1117;
    color: #e6edf3;
    overflow-x: hidden;
}

.app-container {
    display: flex;
    height: 100vh;
    width: 100vw;
}

/* ── Sidebar ── */
.sidebar {
    width: 220px;
    background: #161b22;
    border-right: 1px solid #30363d;
    display: flex;
    flex-direction: column;
    padding: 20px 0;
}

.sidebar-logo {
    font-size: 20px;
    font-weight: 700;
    color: #58a6ff;
    padding: 0 20px 24px;
    letter-spacing: 1px;
}

.nav-item {
    padding: 12px 20px;
    cursor: pointer;
    color: #8b949e;
    font-size: 14px;
    border-left: 3px solid transparent;
    transition: all 0.15s ease;
    display: flex;
    align-items: center;
    gap: 10px;
}

.nav-item:hover {
    color: #e6edf3;
    background: rgba(88, 166, 255, 0.06);
}

.nav-item.active {
    color: #58a6ff;
    border-left-color: #58a6ff;
    background: rgba(88, 166, 255, 0.1);
}

.nav-icon { font-size: 16px; width: 20px; text-align: center; }

/* ── Main content ── */
.main-content {
    flex: 1;
    padding: 32px;
    overflow-y: auto;
}

.page-title {
    font-size: 24px;
    font-weight: 600;
    margin-bottom: 24px;
}

/* ── Cards ── */
.card {
    background: #161b22;
    border: 1px solid #30363d;
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 16px;
}

.card-title {
    font-size: 13px;
    color: #8b949e;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 8px;
}

.card-value {
    font-size: 28px;
    font-weight: 700;
}

.card-value.green { color: #3fb950; }
.card-value.blue { color: #58a6ff; }

/* ── Grid layouts ── */
.balance-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    margin-bottom: 24px;
}

.tx-list { display: flex; flex-direction: column; gap: 8px; }

.tx-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 16px;
    background: #0d1117;
    border-radius: 8px;
    border: 1px solid #30363d;
}

.tx-direction { font-size: 12px; font-weight: 600; }
.tx-direction.send { color: #f85149; }
.tx-direction.receive { color: #3fb950; }
.tx-hash { font-size: 12px; color: #8b949e; font-family: monospace; }
.tx-amount { font-weight: 600; font-size: 14px; }

/* ── Forms ── */
.form-group { margin-bottom: 16px; }

.form-label {
    display: block;
    font-size: 13px;
    color: #8b949e;
    margin-bottom: 6px;
}

.form-input {
    width: 100%;
    padding: 10px 14px;
    background: #0d1117;
    border: 1px solid #30363d;
    border-radius: 8px;
    color: #e6edf3;
    font-size: 14px;
    outline: none;
    transition: border-color 0.15s ease;
}

.form-input:focus { border-color: #58a6ff; }

.form-select {
    width: 100%;
    padding: 10px 14px;
    background: #0d1117;
    border: 1px solid #30363d;
    border-radius: 8px;
    color: #e6edf3;
    font-size: 14px;
    outline: none;
}

.btn {
    padding: 10px 24px;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.15s ease;
}

.btn:hover { opacity: 0.85; }

.btn-primary { background: #58a6ff; color: #0d1117; }
.btn-success { background: #3fb950; color: #0d1117; }
.btn-danger  { background: #f85149; color: #fff; }

.btn-outline {
    background: transparent;
    border: 1px solid #30363d;
    color: #e6edf3;
}

.btn-outline:hover { border-color: #58a6ff; color: #58a6ff; }

/* ── Receive / QR ── */
.qr-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 24px;
}

.qr-box {
    background: #fff;
    border-radius: 12px;
    padding: 16px;
    margin-bottom: 16px;
}

.address-box {
    font-family: monospace;
    font-size: 13px;
    background: #0d1117;
    border: 1px solid #30363d;
    border-radius: 8px;
    padding: 12px 16px;
    word-break: break-all;
    text-align: center;
    color: #58a6ff;
    width: 100%;
    max-width: 400px;
}

.copy-btn {
    margin-top: 12px;
    padding: 8px 20px;
    background: #161b22;
    border: 1px solid #30363d;
    border-radius: 8px;
    color: #e6edf3;
    cursor: pointer;
    font-size: 13px;
}

.copy-btn:hover { border-color: #58a6ff; }

/* ── Settings ── */
.settings-section { margin-bottom: 32px; }

.settings-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 0;
    border-bottom: 1px solid #21262d;
}

.settings-label { font-size: 14px; }
.settings-value { font-size: 14px; color: #8b949e; }

.toggle {
    width: 44px;
    height: 24px;
    background: #30363d;
    border-radius: 12px;
    position: relative;
    cursor: pointer;
    transition: background 0.2s ease;
}

.toggle.on { background: #58a6ff; }

.toggle-dot {
    width: 18px;
    height: 18px;
    background: #fff;
    border-radius: 50%;
    position: absolute;
    top: 3px;
    left: 3px;
    transition: transform 0.2s ease;
}

.toggle.on .toggle-dot { transform: translateX(20px); }

/* ── Status badges ── */
.badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 10px;
    font-size: 11px;
    font-weight: 600;
}

.badge-green { background: rgba(63, 185, 80, 0.15); color: #3fb950; }
.badge-yellow { background: rgba(210, 153, 34, 0.15); color: #d29922; }
.badge-red { background: rgba(248, 81, 73, 0.15); color: #f85149; }

/* ── Chain selector tabs ── */
.chain-tabs {
    display: flex;
    gap: 8px;
    margin-bottom: 20px;
}

.chain-tab {
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    background: #0d1117;
    border: 1px solid #30363d;
    color: #8b949e;
    transition: all 0.15s ease;
}

.chain-tab:hover { color: #e6edf3; }

.chain-tab.active {
    background: rgba(88, 166, 255, 0.1);
    border-color: #58a6ff;
    color: #58a6ff;
}

/* ── Status bar ── */
.status-bar {
    margin-top: auto;
    padding: 12px 20px;
    border-top: 1px solid #30363d;
    font-size: 11px;
    color: #8b949e;
    display: flex;
    align-items: center;
    gap: 6px;
}

.status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #3fb950;
}
"#;

fn main() {
    dioxus::launch(App);
}

/// Root application component.
fn App() -> Element {
    // Active page: "dashboard" | "send" | "receive" | "settings"
    let mut active_page = use_signal(|| "dashboard".to_string());

    // Wallet state
    let mut wallet_initialized = use_signal(|| false);
    let mut eth_address = use_signal(String::new);
    let mut sol_address = use_signal(String::new);
    let eth_balance = use_signal(|| "0".to_string());
    let sol_balance = use_signal(|| "0".to_string());

    // Initialize wallet on first render
    let _init = use_resource(move || async move {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        match xos_sdk::Wallet::from_mnemonic(mnemonic) {
            Ok(wallet) => {
                if let Ok(addr) = wallet.derive_eth_address() {
                    eth_address.set(addr);
                }
                if let Ok(addr) = wallet.derive_solana_address() {
                    sol_address.set(addr);
                }
                wallet_initialized.set(true);
            }
            Err(e) => {
                eprintln!("Wallet init error: {e}");
            }
        }
    });

    rsx! {
        style { {GLOBAL_CSS} }
        div { class: "app-container",
            // ── Sidebar ──
            div { class: "sidebar",
                div { class: "sidebar-logo", "XORION" }

                div {
                    class: if *active_page.read() == "dashboard" { "nav-item active" } else { "nav-item" },
                    onclick: move |_| active_page.set("dashboard".into()),
                    span { class: "nav-icon", "\u{25A0}" }
                    "Dashboard"
                }
                div {
                    class: if *active_page.read() == "send" { "nav-item active" } else { "nav-item" },
                    onclick: move |_| active_page.set("send".into()),
                    span { class: "nav-icon", "\u{2191}" }
                    "Send"
                }
                div {
                    class: if *active_page.read() == "receive" { "nav-item active" } else { "nav-item" },
                    onclick: move |_| active_page.set("receive".into()),
                    span { class: "nav-icon", "\u{2193}" }
                    "Receive"
                }
                div {
                    class: if *active_page.read() == "settings" { "nav-item active" } else { "nav-item" },
                    onclick: move |_| active_page.set("settings".into()),
                    span { class: "nav-icon", "\u{2699}" }
                    "Settings"
                }

                // Status bar at bottom
                div { class: "status-bar",
                    div { class: "status-dot" }
                    if *wallet_initialized.read() {
                        "Wallet connected"
                    } else {
                        "Connecting..."
                    }
                }
            }

            // ── Main content ──
            div { class: "main-content",
                match active_page.read().as_str() {
                    "dashboard" => rsx! {
                        Dashboard {
                            eth_address: eth_address.read().clone(),
                            sol_address: sol_address.read().clone(),
                            eth_balance: eth_balance.read().clone(),
                            sol_balance: sol_balance.read().clone(),
                        }
                    },
                    "send" => rsx! {
                        Send {
                            eth_address: eth_address.read().clone(),
                            sol_address: sol_address.read().clone(),
                        }
                    },
                    "receive" => rsx! {
                        Receive {
                            eth_address: eth_address.read().clone(),
                            sol_address: sol_address.read().clone(),
                        }
                    },
                    "settings" => rsx! { Settings {} },
                    _ => rsx! { Dashboard {
                        eth_address: eth_address.read().clone(),
                        sol_address: sol_address.read().clone(),
                        eth_balance: eth_balance.read().clone(),
                        sol_balance: sol_balance.read().clone(),
                    } },
                }
            }
        }
    }
}
