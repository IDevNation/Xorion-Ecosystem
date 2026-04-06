use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::app::{RefreshStore, SessionStore, SettingsStore, WalletStore};
use crate::explorer::build_explorer_links;
use crate::session::SessionMode;
use crate::session::{
    clear_session, connect_browser_wallet, is_valid_evm_address, save_session, WalletSession,
};

#[component]
pub fn Dashboard() -> Element {
    let settings = use_context::<SettingsStore>();
    let mut session = use_context::<SessionStore>();
    let wallet = use_context::<WalletStore>();
    let mut refresh_nonce = use_context::<RefreshStore>();
    let wallet_data = wallet.read().clone();
    let config = settings.read().clone();
    let session_value = session.read().clone();
    let explorer_ready = matches!(
        session_value.mode,
        SessionMode::BrowserWallet | SessionMode::ImportedAddress
    );
    let explorer_links = build_explorer_links(
        &config.explorer_url,
        if explorer_ready { session_value.address.as_deref() } else { None },
        Some(wallet_data.latest_block.as_str()),
    );
    let mut import_address = use_signal(String::new);
    let mut action_status = use_signal(String::new);
    let is_connected = session.read().address.is_some();

    rsx! {
        div { class: "content-grid",
            section { class: "topbar",
                div {
                    p { class: "topbar-eyebrow", "Xorion Wallet Control Layer" }
                    h1 { class: "topbar-title", "Own the network edge." }
                    p { class: "topbar-copy",
                        "A premium multi-chain wallet surface with live infrastructure posture, high-trust transfer controls, and investor-ready visual depth."
                    }
                }
                div { class: "topbar-side",
                    div { class: "chip", span { "Network" } strong { "{config.network_label}" } }
                    div { class: "chip", span { "Explorer" } strong { "{config.explorer_url}" } }
                    div { class: "chip demo", span { "Mode" } strong { "{wallet_data.session_label}" } }
                }
            }

            div { class: "content-grid dashboard-grid",
                div { class: "stack",
                    section { class: "hero-card",
                        p { class: "eyebrow", "{wallet_data.wallet_name}" }
                        h2 { class: "section-title", "Capital command center" }
                        p { class: "section-copy",
                            "Monitor balances, route payments, and keep your RPC path healthy from one elegant operational surface."
                        }
                        div { class: "wallet-balance", "{wallet_data.primary_balance}" }
                        div { class: "wallet-subline",
                            span { "{wallet_data.secondary_balance}" }
                            span { class: "metric-change", "{wallet_data.change_label}" }
                        }
                        div { class: "button-row",
                            if is_connected {
                                Link { class: "primary-button", to: "/send", "Send assets" }
                                Link { class: "ghost-button", to: "/receive", "Receive funds" }
                            } else {
                                button {
                                    class: "primary-button",
                                    onclick: move |_| {
                                        let mut session = session;
                                        let mut refresh_nonce = refresh_nonce;
                                        let mut action_status = action_status;
                                        spawn(async move {
                                            match connect_browser_wallet().await {
                                                Ok(next_session) => {
                                                    let _ = save_session(&next_session);
                                                    session.set(next_session);
                                                    refresh_nonce.set(refresh_nonce() + 1);
                                                    action_status.set("Browser wallet connected.".to_string());
                                                }
                                                Err(error) => action_status.set(error),
                                            }
                                        });
                                    },
                                    "Connect browser wallet"
                                }
                                button {
                                    class: "ghost-button",
                                    onclick: move |_| {
                                        let preview = WalletSession::preview();
                                        let _ = save_session(&preview);
                                        session.set(preview);
                                        refresh_nonce.set(refresh_nonce() + 1);
                                        action_status.set("Preview wallet created for this browser.".to_string());
                                    },
                                    "Create preview wallet"
                                }
                            }
                        }
                        div { class: "button-row",
                            div { class: "address-pill", "Vault: {wallet_data.short_address}" }
                            div { class: "address-pill", "{wallet_data.synced_at}" }
                        }
                        if let Some(address_link) = explorer_links.address.clone() {
                            div { class: "button-row",
                                a {
                                    class: "ghost-button",
                                    href: "{address_link}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "View address in explorer"
                                }
                                if let Some(block_link) = explorer_links.block.clone() {
                                    a {
                                        class: "ghost-button",
                                        href: "{block_link}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "View latest block"
                                    }
                                }
                            }
                        }
                        div { class: "trust-note",
                            if wallet_data.is_syncing {
                                "The wallet is refreshing live account and network data from the configured EVM RPC."
                            } else if wallet_data.is_live {
                                "Thin real path active: the address and RPC metadata are live. Sending is still preview-safe and does not broadcast a transaction."
                            } else if matches!(session_value.mode, SessionMode::PreviewWallet) {
                                "Preview wallet mode is local to this browser. Explorer links stay disabled here because the preview address is not a trusted external account."
                            } else {
                                "This wallet is not connected to a live account yet. Connect a browser wallet, import an address, or create a local preview wallet."
                            }
                        }
                        div { class: "field-grid",
                            div { class: "field-group",
                                label { class: "field-label", "Import watch-only address" }
                                input {
                                    class: "field-input",
                                    r#type: "text",
                                    placeholder: "0x... address",
                                    value: "{import_address.read()}",
                                    oninput: move |evt| import_address.set(evt.value())
                                }
                                p { class: "field-help", "This adds a real address for balance and network reads without handling private keys." }
                            }
                            div { class: "button-row",
                                button {
                                    class: "ghost-button",
                                    onclick: move |_| {
                                        let value = import_address.read().trim().to_string();
                                        if !is_valid_evm_address(&value) {
                                            action_status.set("Enter a valid 0x-prefixed EVM address.".to_string());
                                            return;
                                        }
                                        let imported = WalletSession::imported(value);
                                        let _ = save_session(&imported);
                                        session.set(imported);
                                        refresh_nonce.set(refresh_nonce() + 1);
                                        action_status.set("Watch-only address imported.".to_string());
                                    },
                                    "Import address"
                                }
                                if is_connected {
                                    button {
                                        class: "ghost-button",
                                        onclick: move |_| {
                                            let _ = clear_session();
                                            session.set(WalletSession::disconnected());
                                            refresh_nonce.set(refresh_nonce() + 1);
                                            action_status.set("Wallet session cleared from this browser.".to_string());
                                        },
                                        "Disconnect"
                                    }
                                        button {
                                            class: "ghost-button",
                                            onclick: move |_| {
                                            refresh_nonce.set(refresh_nonce() + 1);
                                            action_status.set("Refreshing live wallet data...".to_string());
                                        },
                                        "Refresh"
                                    }
                                }
                            }
                        }
                        if !action_status.read().is_empty() {
                            div { class: "message warn", "{action_status.read()}" }
                        }
                    }

                    div { class: "stats-grid",
                        section { class: "panel-card metric",
                            span { class: "metric-label", "Wallet Summary" }
                            strong { class: "metric-value", "{wallet_data.secondary_balance}" }
                            span { class: "metric-change", "{wallet_data.change_label}" }
                            p { class: "section-copy",
                                "Primary treasury reserve held inside the active MVP wallet profile."
                            }
                        }
                        section { class: "panel-card metric",
                            span { class: "metric-label", "Connected RPC" }
                            strong { class: "metric-value", "{wallet_data.rpc_state}" }
                            span { class: "metric-change", "{wallet_data.network_health}" }
                            p { class: "section-copy", "{config.rpc_url}" }
                        }
                    }

                    section { class: "panel-card",
                        h3 { class: "section-title", "Activity" }
                        p { class: "section-copy", "Live transaction history is not wired yet in this phase, so the wallet shows an honest empty state instead of mixed mock data." }
                        if wallet_data.activity.is_empty() {
                            div { class: "message",
                                "No live activity feed is available yet. Explorer integration is the next clean step for account and transaction history."
                            }
                        } else {
                            div { class: "activity-list",
                                {wallet_data.activity.iter().map(|item| rsx!(
                                    div {
                                        key: "{item.title}-{item.subtitle}",
                                        class: "activity-item",
                                        div { class: "activity-copy",
                                            div { class: "activity-title", "{item.title}" }
                                            div { class: "activity-meta", "{item.subtitle}" }
                                        }
                                        div { class: "activity-amount",
                                            strong { "{item.amount}" }
                                            span { "{item.state}" }
                                        }
                                        div { class: "pill {item.direction}", "{item.direction}" }
                                    }
                                ))}
                            }
                        }
                    }
                }

                div { class: "stack",
                    section { class: "panel-card",
                        h3 { class: "section-title", "Account detail" }
                        p { class: "section-copy", "A read-only trust view for the active account, with direct explorer visibility." }
                        div { class: "detail-list",
                            div { class: "detail-item", label { "Address" } strong { "{wallet_data.short_address}" } }
                            div { class: "detail-item", label { "Session mode" } strong { "{wallet_data.session_label}" } }
                            div { class: "detail-item", label { "Chain" } strong { "{wallet_data.chain_name}" } }
                            div { class: "detail-item", label { "Latest block" } strong { "{wallet_data.latest_block}" } }
                        }
                        div { class: "button-row",
                            if let Some(address_link) = explorer_links.address.clone() {
                                a {
                                    class: "ghost-button",
                                    href: "{address_link}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "Open account detail"
                                }
                            }
                            if let Some(home_link) = explorer_links.home.clone() {
                                a {
                                    class: "ghost-button",
                                    href: "{home_link}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "Open explorer home"
                                }
                            }
                        }
                        if !explorer_ready {
                            div { class: "message",
                                "Explorer account links appear only for imported addresses or connected browser wallets."
                            }
                        } else if explorer_links.home.is_none() {
                            div { class: "message warn",
                                "Explorer links are unavailable until Settings contains a clean explorer base URL."
                            }
                        }
                    }

                    section { class: "panel-card",
                        h3 { class: "section-title", "Network posture" }
                        div { class: "status-list",
                            div { class: "status-item", label { "RPC health" } strong { class: "status-good", "{wallet_data.rpc_state}" } }
                            div { class: "status-item", label { "Environment" } strong { "{config.environment}" } }
                            div { class: "status-item", label { "Chain" } strong { "{wallet_data.chain_name}" } }
                            div { class: "status-item", label { "Chain ID" } strong { "{wallet_data.chain_id}" } }
                            div { class: "status-item", label { "Latest block" } strong { "{wallet_data.latest_block}" } }
                            div { class: "status-item", label { "Sync marker" } strong { class: "status-warn", "{wallet_data.synced_at}" } }
                        }
                    }

                    section { class: "panel-card",
                        h3 { class: "section-title", "Quick actions" }
                        p { class: "section-copy", "Use the main wallet actions without leaving the premium command view." }
                        div { class: "button-row",
                            Link { class: "primary-button", to: "/send", "Initiate send" }
                            Link { class: "ghost-button", to: "/receive", "Open receive" }
                            Link { class: "ghost-button", to: "/settings", "Edit endpoints" }
                            if let Some(address_link) = explorer_links.address.clone() {
                                a {
                                    class: "ghost-button",
                                    href: "{address_link}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "Open explorer"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
