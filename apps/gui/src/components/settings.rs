#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let mut rpc_eth = use_signal(|| "https://eth.llamarpc.com".to_string());
    let mut rpc_sol = use_signal(|| "https://api.mainnet-beta.solana.com".to_string());
    let mut dark_theme = use_signal(|| true);
    let mut auto_lock = use_signal(|| true);
    let mut notifications = use_signal(|| true);
    let mut saved = use_signal(|| false);

    let on_save = move |_| {
        saved.set(true);
        // In production this would persist settings to disk
    };

    rsx! {
        h1 { class: "page-title", "Settings" }

        // ── Network Configuration ──
        div { class: "settings-section",
            h3 { style: "font-size: 14px; color: #8b949e; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 12px;",
                "Network Configuration"
            }
            div { class: "card",
                div { class: "form-group",
                    label { class: "form-label", "Ethereum RPC Endpoint" }
                    input {
                        class: "form-input",
                        r#type: "text",
                        value: "{rpc_eth}",
                        oninput: move |e| { rpc_eth.set(e.value()); saved.set(false); },
                    }
                }
                div { class: "form-group", style: "margin-bottom: 0;",
                    label { class: "form-label", "Solana RPC Endpoint" }
                    input {
                        class: "form-input",
                        r#type: "text",
                        value: "{rpc_sol}",
                        oninput: move |e| { rpc_sol.set(e.value()); saved.set(false); },
                    }
                }
            }
        }

        // ── Appearance ──
        div { class: "settings-section",
            h3 { style: "font-size: 14px; color: #8b949e; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 12px;",
                "Appearance"
            }
            div { class: "card",
                div { class: "settings-row",
                    div { class: "settings-label", "Dark Theme" }
                    div {
                        class: if *dark_theme.read() { "toggle on" } else { "toggle" },
                        onclick: move |_| { let v = *dark_theme.read(); dark_theme.set(!v); saved.set(false); },
                        div { class: "toggle-dot" }
                    }
                }
            }
        }

        // ── Security ──
        div { class: "settings-section",
            h3 { style: "font-size: 14px; color: #8b949e; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 12px;",
                "Security"
            }
            div { class: "card",
                div { class: "settings-row",
                    div {
                        div { class: "settings-label", "Auto-Lock Wallet" }
                        div { style: "font-size: 12px; color: #8b949e;", "Lock after 5 minutes of inactivity" }
                    }
                    div {
                        class: if *auto_lock.read() { "toggle on" } else { "toggle" },
                        onclick: move |_| { let v = *auto_lock.read(); auto_lock.set(!v); saved.set(false); },
                        div { class: "toggle-dot" }
                    }
                }
                div { class: "settings-row", style: "border-bottom: none;",
                    div {
                        div { class: "settings-label", "Transaction Notifications" }
                        div { style: "font-size: 12px; color: #8b949e;", "Alert on incoming transactions" }
                    }
                    div {
                        class: if *notifications.read() { "toggle on" } else { "toggle" },
                        onclick: move |_| { let v = *notifications.read(); notifications.set(!v); saved.set(false); },
                        div { class: "toggle-dot" }
                    }
                }
            }
        }

        // ── About ──
        div { class: "settings-section",
            h3 { style: "font-size: 14px; color: #8b949e; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 12px;",
                "About"
            }
            div { class: "card",
                div { class: "settings-row",
                    div { class: "settings-label", "Version" }
                    div { class: "settings-value", "Xorion Web3 OS v0.4.0" }
                }
                div { class: "settings-row",
                    div { class: "settings-label", "SDK" }
                    div { class: "settings-value", "xorion-wallet-sdk 0.4.0" }
                }
                div { class: "settings-row",
                    div { class: "settings-label", "Runtime" }
                    div { class: "settings-value", "Redox OS / Linux" }
                }
                div { class: "settings-row", style: "border-bottom: none;",
                    div { class: "settings-label", "License" }
                    div { class: "settings-value", "MIT" }
                }
            }
        }

        // ── Save button ──
        div { style: "display: flex; gap: 12px; align-items: center;",
            button {
                class: "btn btn-primary",
                onclick: on_save,
                "Save Settings"
            }
            if *saved.read() {
                span { style: "font-size: 13px; color: #3fb950;", "Settings saved" }
            }
        }

        // ── Danger zone ──
        div { class: "settings-section", style: "margin-top: 32px;",
            h3 { style: "font-size: 14px; color: #f85149; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 12px;",
                "Danger Zone"
            }
            div { class: "card", style: "border-color: #f8514933;",
                div { style: "display: flex; justify-content: space-between; align-items: center;",
                    div {
                        div { class: "settings-label", "Reset Wallet" }
                        div { style: "font-size: 12px; color: #8b949e;", "Remove all wallet data from this device" }
                    }
                    button { class: "btn btn-danger", "Reset" }
                }
            }
        }
    }
}
