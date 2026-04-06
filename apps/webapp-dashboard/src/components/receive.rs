use dioxus::prelude::*;

use crate::app::{SessionStore, SettingsStore, WalletStore};
use crate::explorer::build_explorer_links;
use crate::session::SessionMode;

#[component]
pub fn ReceiveScreen() -> Element {
    let settings = use_context::<SettingsStore>();
    let session = use_context::<SessionStore>();
    let wallet = use_context::<WalletStore>();
    let mut status = use_signal(String::new);
    let address = wallet.read().full_address.clone();
    let session_value = session.read().clone();
    let explorer_ready = matches!(
        session_value.mode,
        SessionMode::BrowserWallet | SessionMode::ImportedAddress
    );
    let explorer_links = build_explorer_links(
        &settings.read().explorer_url,
        if explorer_ready { session_value.address.as_deref() } else { None },
        Some(wallet.read().latest_block.as_str()),
    );

    rsx! {
        div { class: "content-grid",
            section { class: "topbar",
                div {
                    p { class: "topbar-eyebrow", "Inbound capital" }
                    h1 { class: "topbar-title", "Receive into your vault." }
                    p { class: "topbar-copy",
                        "Share a clean funding surface, copy the active address, and keep every inbound transfer aligned with your configured network route."
                    }
                }
                div { class: "topbar-side",
                    div { class: "chip", span { "Network" } strong { "{settings.read().network_label}" } }
                }
            }

            div { class: "receive-grid",
                section { class: "panel-card stack",
                    div {
                        h2 { class: "section-title", "Wallet address" }
                        p { class: "section-copy", "Use this address for deposits into the current MVP wallet profile." }
                    }

                    div { class: "receive-address", "{address}" }

                    div { class: "button-row",
                        button {
                            class: "copy-button",
                            onclick: move |_| {
                                #[cfg(target_arch = "wasm32")]
                                {
                                    if let Some(window) = web_sys::window() {
                                        let clipboard = window.navigator().clipboard();
                                        let _ = clipboard.write_text(&address);
                                        status.set("Address copied to clipboard.".to_string());
                                    } else {
                                        status.set("Browser clipboard is unavailable.".to_string());
                                    }
                                }

                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    status.set("Clipboard is only available in the browser build.".to_string());
                                }
                            },
                            "Copy address"
                        }
                        if let Some(address_link) = explorer_links.address.clone() {
                            a {
                                class: "ghost-button",
                                href: "{address_link}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "View in explorer"
                            }
                        }
                    }

                    if !status.read().is_empty() {
                        div { class: "message success", "{status.read()}" }
                    }

                    if matches!(session_value.mode, SessionMode::PreviewWallet) {
                        div { class: "trust-note",
                            "Preview wallet mode is local-only. Copy works for UI review, but explorer verification is intentionally disabled."
                        }
                    } else if session_value.address.is_none() {
                        div { class: "trust-note",
                            "No live wallet session is connected yet. Use the dashboard to connect a browser wallet or import an address."
                        }
                    }
                }

                section { class: "panel-card stack",
                    div {
                        h2 { class: "section-title", "QR funding panel" }
                        p { class: "section-copy", "A polished QR placeholder keeps the receive experience investor-ready without heavy extra dependencies." }
                    }

                    div { class: "qr-shell",
                        div { class: "qr-grid",
                            span { class: "on" } span { class: "on" } span {} span { class: "on" } span {} span { class: "on" } span { class: "on" }
                            span { class: "on" } span {} span { class: "on" } span {} span { class: "on" } span {} span { class: "on" }
                            span {} span { class: "on" } span { class: "on" } span {} span { class: "on" } span { class: "on" } span {}
                            span { class: "on" } span {} span {} span { class: "on" } span {} span {} span { class: "on" }
                            span {} span { class: "on" } span { class: "on" } span {} span { class: "on" } span { class: "on" } span {}
                            span { class: "on" } span {} span { class: "on" } span {} span { class: "on" } span {} span { class: "on" }
                            span { class: "on" } span { class: "on" } span {} span { class: "on" } span {} span { class: "on" } span { class: "on" }
                        }
                    }

                    div { class: "detail-list",
                        div { class: "detail-item", label { "Explorer" } strong { "{settings.read().explorer_url}" } }
                        div { class: "detail-item", label { "Address label" } strong { "{wallet.read().wallet_name}" } }
                        div { class: "detail-item", label { "Latest block" } strong { "{wallet.read().latest_block}" } }
                    }
                    if explorer_ready {
                        div { class: "trust-note",
                            "This address is real for the active wallet session. Use the explorer link for external account verification and future transaction history."
                        }
                    } else if explorer_links.home.is_none() {
                        div { class: "message warn",
                            "Set a clean explorer base URL in Settings to enable external account verification links."
                        }
                    }
                }
            }
        }
    }
}
