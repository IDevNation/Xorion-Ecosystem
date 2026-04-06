use dioxus::prelude::*;

use crate::app::{SessionStore, SettingsStore, WalletStore};

#[component]
pub fn SendScreen() -> Element {
    let settings = use_context::<SettingsStore>();
    let session = use_context::<SessionStore>();
    let wallet = use_context::<WalletStore>();

    let mut recipient = use_signal(String::new);
    let mut amount = use_signal(String::new);
    let mut asset = use_signal(|| "XOR".to_string());
    let mut network = use_signal(|| settings.read().network_label.clone());
    let mut status = use_signal(String::new);

    let recipient_value = recipient.read().clone();
    let amount_value = amount.read().clone();
    let amount_number = amount_value.parse::<f64>().ok();
    let recipient_error = if recipient_value.is_empty() || recipient_value.len() >= 12 {
        None
    } else {
        Some("Enter a longer wallet address.".to_string())
    };
    let amount_error = if amount_value.is_empty() {
        None
    } else if amount_number.unwrap_or(0.0) <= 0.0 {
        Some("Amount must be greater than zero.".to_string())
    } else {
        None
    };
    let can_submit =
        recipient_error.is_none() && amount_error.is_none() && !recipient_value.is_empty() && !amount_value.is_empty();

    let estimated_fee = match network.read().as_str() {
        "Solana Devnet" => "0.0004 SOL",
        "Ethereum Mainnet" => "0.0028 ETH",
        _ => "0.12 XOR",
    };

    rsx! {
        div { class: "content-grid",
            section { class: "topbar",
                div {
                    p { class: "topbar-eyebrow", "Outbound transfer" }
                    h1 { class: "topbar-title", "Send with confidence." }
                    p { class: "topbar-copy",
                        "Premium transfer controls, clean validation, and a live transaction summary before you confirm any movement."
                    }
                }
                div { class: "topbar-side",
                    div { class: "chip", span { "Wallet" } strong { "{wallet.read().short_address}" } }
                }
            }

            div { class: "send-grid",
                section { class: "panel-card",
                    h2 { class: "section-title", "Transfer details" }
                    p { class: "section-copy", "Enter the destination, amount, and preferred route." }

                    div { class: "field-grid",
                        div { class: "field-group",
                            label { class: "field-label", "Recipient address" }
                            input {
                                class: if recipient_error.is_some() { "field-input field-error" } else { "field-input" },
                                r#type: "text",
                                placeholder: "0x... or wallet destination",
                                value: "{recipient_value}",
                                oninput: move |evt| recipient.set(evt.value())
                            }
                            if let Some(ref error) = recipient_error {
                                p { class: "field-help error", "{error}" }
                            } else {
                                p { class: "field-help", "Paste a wallet or infrastructure destination address." }
                            }
                        }

                        div { class: "two-up",
                            div { class: "field-group",
                                label { class: "field-label", "Amount" }
                                input {
                                    class: if amount_error.is_some() { "field-input field-error" } else { "field-input" },
                                    r#type: "number",
                                    step: "0.0001",
                                    placeholder: "0.00",
                                    value: "{amount_value}",
                                    oninput: move |evt| amount.set(evt.value())
                                }
                                if let Some(ref error) = amount_error {
                                    p { class: "field-help error", "{error}" }
                                } else {
                                    p { class: "field-help", "Available balance: {wallet.read().primary_balance}" }
                                }
                            }

                            div { class: "field-group",
                                label { class: "field-label", "Asset" }
                                select {
                                    class: "field-select",
                                    value: "{asset.read()}",
                                    onchange: move |evt| asset.set(evt.value()),
                                    option { value: "XOR", "XOR" }
                                    option { value: "USDX", "USDX" }
                                    option { value: "ETH", "ETH" }
                                }
                                p { class: "field-help", "Choose the asset route for this MVP transfer." }
                            }
                        }

                        div { class: "field-group",
                            label { class: "field-label", "Network" }
                            select {
                                class: "field-select",
                                value: "{network.read()}",
                                onchange: move |evt| network.set(evt.value()),
                                option { value: "Xorion Testnet", "Xorion Testnet" }
                                option { value: "Ethereum Mainnet", "Ethereum Mainnet" }
                                option { value: "Solana Devnet", "Solana Devnet" }
                            }
                            p { class: "field-help", "Current RPC route: {settings.read().rpc_url}" }
                        }
                    }

                    div { class: "button-row",
                        button {
                            class: if can_submit { "primary-button" } else { "primary-button disabled" },
                            disabled: !can_submit,
                            onclick: move |_| {
                                status.set(format!(
                                    "Preview only: {} {} has been prepared for review to {} via {}. No transaction was sent.",
                                    amount.read(),
                                    asset.read(),
                                    recipient.read(),
                                    network.read()
                                ));
                            },
                            "Prepare transaction"
                        }
                    }

                    if !status.read().is_empty() {
                        div { class: "message warn", "{status.read()}" }
                    }

                    if session.read().address.is_none() {
                        div { class: "trust-note",
                            "Connect a browser wallet or import an address on the dashboard first. Send remains preview-only in this phase."
                        }
                    }
                }

                section { class: "panel-card",
                    h2 { class: "section-title", "Transaction summary" }
                    p { class: "section-copy", "A clean pre-flight view before capital leaves the wallet." }
                    div { class: "summary-list",
                        div {
                            class: "summary-item",
                            label { "Destination" }
                            strong {
                                if recipient_value.is_empty() {
                                    "Awaiting input"
                                } else {
                                    "{recipient_value}"
                                }
                            }
                        }
                        div {
                            class: "summary-item",
                            label { "Send amount" }
                            strong {
                                if amount_value.is_empty() {
                                    "--"
                                } else {
                                    "{amount_value} {asset.read()}"
                                }
                            }
                        }
                        div { class: "summary-item", label { "Network route" } strong { "{network.read()}" } }
                        div { class: "summary-item", label { "Estimated fee" } strong { "{estimated_fee}" } }
                        div { class: "summary-item", label { "Explorer" } strong { "{settings.read().explorer_url}" } }
                    }

                    div { class: "summary-highlight",
                        strong { "Validation posture" }
                        p {
                            if can_submit {
                                "Ready for review in preview mode. Inputs look complete and the route is configured."
                            } else {
                                "Complete the destination and amount fields to unlock the primary transfer action."
                            }
                        }
                    }

                    div { class: "trust-note",
                        "Safety note: this MVP does not broadcast a real blockchain transaction yet. Treat this screen as a product-preview transfer flow."
                    }
                }
            }
        }
    }
}
