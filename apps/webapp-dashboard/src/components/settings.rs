use dioxus::prelude::*;

use crate::app::{save_settings, AppSettings, SettingsStore, WalletStore};

#[component]
pub fn Settings() -> Element {
    let mut settings = use_context::<SettingsStore>();
    let wallet = use_context::<WalletStore>();
    let initial = settings.read().clone();

    let mut rpc_url = use_signal(|| initial.rpc_url.clone());
    let mut explorer_url = use_signal(|| initial.explorer_url.clone());
    let mut environment = use_signal(|| initial.environment.clone());
    let mut network_label = use_signal(|| initial.network_label.clone());
    let mut status = use_signal(String::new);
    let is_valid_url = |value: &str| {
        let trimmed = value.trim();
        trimmed.starts_with("http://") || trimmed.starts_with("https://")
    };

    rsx! {
        div { class: "content-grid",
            section { class: "topbar",
                div {
                    p { class: "topbar-eyebrow", "Infrastructure settings" }
                    h1 { class: "topbar-title", "Tune the control plane." }
                    p { class: "topbar-copy",
                        "Adjust the RPC and explorer endpoints for the MVP wallet while keeping your network profile visible and easy to trust."
                    }
                }
                div { class: "topbar-side",
                    div { class: "chip", span { "Session" } strong { "{wallet.read().rpc_state}" } }
                }
            }

            div { class: "settings-grid",
                section { class: "panel-card",
                    h2 { class: "section-title", "Endpoint configuration" }
                    p { class: "section-copy", "These values define how the MVP wallet reads an EVM-compatible RPC and prepares future explorer handoff." }

                    div { class: "field-grid",
                        div { class: "field-group",
                            label { class: "field-label", "RPC URL" }
                            input {
                                class: "field-input",
                                r#type: "text",
                                value: "{rpc_url.read()}",
                                oninput: move |evt| rpc_url.set(evt.value())
                            }
                            p { class: "field-help", "Use an EVM-compatible JSON-RPC endpoint. Non-EVM endpoints will fail safely." }
                        }
                        div { class: "field-group",
                            label { class: "field-label", "Explorer URL" }
                            input {
                                class: "field-input",
                                r#type: "text",
                                value: "{explorer_url.read()}",
                                oninput: move |evt| explorer_url.set(evt.value())
                            }
                        }
                        div { class: "two-up",
                            div { class: "field-group",
                                label { class: "field-label", "Environment" }
                                input {
                                    class: "field-input",
                                    r#type: "text",
                                    value: "{environment.read()}",
                                    oninput: move |evt| environment.set(evt.value())
                                }
                            }
                            div { class: "field-group",
                                label { class: "field-label", "Network label" }
                                input {
                                    class: "field-input",
                                    r#type: "text",
                                    value: "{network_label.read()}",
                                    oninput: move |evt| network_label.set(evt.value())
                                }
                            }
                        }
                    }

                    div { class: "button-row",
                        button {
                            class: "primary-button",
                            onclick: move |_| {
                                if !is_valid_url(&rpc_url.read()) {
                                    status.set("RPC URL must start with http:// or https://".to_string());
                                    return;
                                }
                                if !is_valid_url(&explorer_url.read()) {
                                    status.set("Explorer URL must start with http:// or https://".to_string());
                                    return;
                                }
                                let next = AppSettings {
                                    rpc_url: rpc_url.read().clone(),
                                    explorer_url: explorer_url.read().clone(),
                                    environment: environment.read().clone(),
                                    network_label: network_label.read().clone(),
                                };
                                settings.set(next.clone());
                                match save_settings(&next) {
                                    Ok(_) => status.set("Settings saved for this browser session.".to_string()),
                                    Err(error) => status.set(error),
                                }
                            },
                            "Save settings"
                        }
                    }

                    if !status.read().is_empty() {
                        div {
                            class: if status.read().contains("saved") { "message success" } else { "message warn" },
                            "{status.read()}"
                        }
                    }

                    p { class: "settings-note",
                        "This MVP keeps the settings simple, browser-safe, and easy to demo without pulling in native-only dependencies."
                    }
                }

                section { class: "panel-card stack",
                    div {
                        h2 { class: "section-title", "Environment info" }
                        p { class: "section-copy", "High-level operational data for a confident investor or product demo." }
                    }
                    div { class: "detail-list",
                        div { class: "detail-item", label { "Wallet profile" } strong { "{wallet.read().wallet_name}" } }
                        div { class: "detail-item", label { "Network health" } strong { "{wallet.read().network_health}" } }
                        div { class: "detail-item", label { "Current sync" } strong { "{wallet.read().synced_at}" } }
                        div { class: "detail-item", label { "Preview mode" } strong { "Browser-first MVP" } }
                    }
                }
            }
        }
    }
}
