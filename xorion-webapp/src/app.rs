use dioxus::prelude::*;
use crate::components::{dashboard::Dashboard, send::SendScreen, receive::ReceiveScreen, settings::Settings};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[layout(Header)]
        #[route("/")]
        Dashboard {},
        #[route("/send")]
        Send {},
        #[route("/receive")]
        Receive {},
        #[route("/settings")]
        Settings {},
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
fn Header() -> Element {
    rsx! {
        header {
            style: "display: flex; justify-content: space-between; align-items: center; padding: 1rem 2rem; background: #1a1a1a; border-bottom: 1px solid #2a2a2a;",
            div {
                style: "display: flex; align-items: center; gap: 0.5rem;",
                span { style: "font-size: 1.5rem; color: #6c47ff;", "⬡" }
                h1 { style: "margin: 0; font-size: 1.5rem;", "Xorion Wallet" }
            }
            nav {
                style: "display: flex; gap: 1rem;",
                Link { to: Route::Dashboard {}, 
                    button { class: "nav-btn", "Dashboard" }
                }
                Link { to: Route::Send {}, 
                    button { class: "nav-btn", "Send" }
                }
                Link { to: Route::Receive {}, 
                    button { class: "nav-btn", "Receive" }
                }
                Link { to: Route::Settings {}, 
                    button { class: "nav-btn", "Settings" }
                }
            }
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;",
            h1 { "404 - Page Not Found" }
            p { "The page you're looking for doesn't exist." }
            Link { to: Route::Dashboard {}, 
                button { class: "btn-primary", "Go Home" }
            }
        }
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        style {
            r#"
            * { margin: 0; padding: 0; box-sizing: border-box; }
            body { 
                font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
                background: #0a0a0a;
                color: #ffffff;
            }
            .nav-btn {
                background: transparent;
                border: none;
                color: #a0a0a0;
                padding: 0.5rem 1rem;
                cursor: pointer;
                transition: color 0.3s;
            }
            .nav-btn:hover { color: #6c47ff; }
            .btn-primary {
                background: linear-gradient(135deg, #6c47ff, #8b69ff);
                color: white;
                border: none;
                padding: 0.75rem 1.5rem;
                border-radius: 8px;
                cursor: pointer;
                font-weight: 600;
            }
            .btn-primary:hover { transform: translateY(-2px); }
            .card {
                background: #161616;
                border: 1px solid #2a2a2a;
                border-radius: 12px;
                padding: 1.5rem;
                margin: 1rem 0;
            }
            input, select {
                background: #1a1a1a;
                border: 1px solid #2a2a2a;
                color: white;
                padding: 0.75rem;
                border-radius: 6px;
                width: 100%;
                margin: 0.5rem 0;
            }
            label { color: #a0a0a0; font-size: 0.9rem; }
            "#
        }
        Router::<Route> {}
    }
}
