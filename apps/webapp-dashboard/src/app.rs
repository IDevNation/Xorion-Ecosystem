use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::{
    dashboard::Dashboard as DashboardScreen,
    receive::ReceiveScreen,
    send::SendScreen,
    settings::Settings as SettingsScreen,
};
use crate::rpc::{fetch_overview, RpcOverview};
use crate::session::{load_session, WalletSession};

pub type SettingsStore = Signal<AppSettings>;
pub type WalletStore = Signal<WalletSnapshot>;
pub type SessionStore = Signal<WalletSession>;
pub type RefreshStore = Signal<u64>;

const APP_CSS: &str = r#"
:root {
    --bg: #f4f8ff;
    --bg-elevated: rgba(255, 255, 255, 0.8);
    --bg-panel: rgba(255, 255, 255, 0.72);
    --border: rgba(114, 135, 201, 0.16);
    --border-strong: rgba(95, 114, 205, 0.28);
    --text: #131a33;
    --muted: #66718f;
    --soft: #7a85a2;
    --blue: #63a4ff;
    --purple: #8f6dff;
    --red: #ff6b7a;
    --cyan: #74e6ff;
    --success: #79f7c4;
    --shadow: 0 30px 80px rgba(92, 121, 209, 0.18);
    --shadow-soft: 0 18px 50px rgba(92, 121, 209, 0.12);
    --radius-xl: 32px;
    --radius-lg: 28px;
    --radius-md: 18px;
    --radius-pill: 999px;
    --space-1: 8px;
    --space-2: 12px;
    --space-3: 18px;
    --space-4: 24px;
    --space-5: 28px;
    --type-display: clamp(2rem, 5vw, 4.6rem);
    --type-body: 1rem;
}

* { box-sizing: border-box; }

html, body {
    margin: 0;
    min-height: 100%;
    background:
        radial-gradient(circle at top left, rgba(143, 109, 255, 0.16), transparent 30%),
        radial-gradient(circle at top right, rgba(99, 164, 255, 0.14), transparent 28%),
        radial-gradient(circle at bottom right, rgba(255, 107, 122, 0.12), transparent 24%),
        linear-gradient(180deg, #fbfdff 0%, #f3f8ff 40%, #edf3ff 100%);
    color: var(--text);
    font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
}

body { overflow-x: hidden; }
a { color: inherit; text-decoration: none; }
button, input, select { font: inherit; }

.app-shell { position: relative; min-height: 100vh; overflow: hidden; }
.ambient { position: absolute; inset: 0; pointer-events: none; }
.ambient::before, .ambient::after {
    content: "";
    position: absolute;
    border-radius: 999px;
    filter: blur(80px);
    opacity: 0.8;
}
.ambient::before {
    width: 420px;
    height: 420px;
    top: -120px;
    left: -80px;
    background: rgba(143, 109, 255, 0.18);
    animation: floatGlow 18s ease-in-out infinite;
}
.ambient::after {
    width: 360px;
    height: 360px;
    right: -70px;
    top: 160px;
    background: rgba(99, 164, 255, 0.18);
    animation: floatGlow 22s ease-in-out infinite reverse;
}

.app-frame {
    position: relative;
    z-index: 1;
    display: grid;
    grid-template-columns: 280px 1fr;
    min-height: 100vh;
}

.sidebar {
    position: relative;
    padding: 28px 22px;
    background: linear-gradient(180deg, rgba(255, 255, 255, 0.68), rgba(245, 248, 255, 0.66));
    border-right: 1px solid rgba(114, 135, 201, 0.12);
    backdrop-filter: blur(18px);
}

.sidebar::after {
    content: "";
    position: absolute;
    inset: 18px 12px auto;
    height: 1px;
    background: linear-gradient(90deg, rgba(143, 109, 255, 0.4), rgba(99, 164, 255, 0.05));
}

.brand-lockup { margin-bottom: 28px; }
.brand-pill {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    border-radius: var(--radius-pill);
    background: rgba(255, 255, 255, 0.74);
    border: 1px solid rgba(114, 135, 201, 0.14);
    box-shadow: inset 0 1px 0 rgba(255,255,255,0.55);
}
.brand-mark {
    width: 36px;
    height: 36px;
    display: grid;
    place-items: center;
    border-radius: 12px;
    color: white;
    font-weight: 800;
    letter-spacing: 0.08em;
    background: linear-gradient(135deg, rgba(143, 109, 255, 0.95), rgba(99, 164, 255, 0.95));
    box-shadow: 0 18px 35px rgba(112, 92, 255, 0.35);
}
.brand-copy h1 {
    margin: 0;
    font-size: 1rem;
    letter-spacing: 0.1em;
    text-transform: uppercase;
}
.brand-copy p {
    margin: 2px 0 0;
    color: var(--muted);
    font-size: 0.8rem;
}
.sidebar-blurb {
    margin: 18px 0 24px;
    color: var(--muted);
    line-height: 1.6;
    font-size: 0.94rem;
}

.nav-stack { display: flex; flex-direction: column; gap: 10px; }
.nav-link {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px;
    border-radius: 18px;
    color: var(--muted);
    border: 1px solid transparent;
    background: rgba(255, 255, 255, 0.48);
    transition: transform 180ms ease, border-color 180ms ease, background 180ms ease, color 180ms ease, box-shadow 180ms ease;
}
.nav-link:hover, .nav-link.active {
    transform: translateX(4px);
    color: var(--text);
    border-color: rgba(143, 109, 255, 0.24);
    background: linear-gradient(135deg, rgba(143, 109, 255, 0.12), rgba(99, 164, 255, 0.08));
    box-shadow: 0 12px 30px rgba(16, 20, 36, 0.4);
}
.nav-meta { display: flex; flex-direction: column; gap: 4px; }
.nav-title { font-size: 0.98rem; font-weight: 600; }
.nav-subtitle { color: var(--soft); font-size: 0.76rem; }
.nav-link.active .nav-subtitle, .nav-link:hover .nav-subtitle { color: rgba(236, 240, 255, 0.72); }
.nav-badge {
    width: 36px;
    height: 36px;
    border-radius: 14px;
    display: grid;
    place-items: center;
    background: rgba(255,255,255,0.68);
    border: 1px solid rgba(114, 135, 201, 0.12);
    color: var(--text);
    font-size: 0.8rem;
}

.sidebar-status {
    margin-top: 28px;
    padding: 18px;
    border-radius: 24px;
    background: linear-gradient(180deg, rgba(255, 255, 255, 0.82), rgba(247, 249, 255, 0.75));
    border: 1px solid rgba(114, 135, 201, 0.14);
    box-shadow: var(--shadow);
}
.status-row { display: flex; align-items: center; gap: 10px; font-size: 0.92rem; color: var(--text); }
.status-dot {
    width: 10px;
    height: 10px;
    border-radius: 999px;
    background: var(--success);
    box-shadow: 0 0 16px rgba(121, 247, 196, 0.8);
}
.status-copy {
    margin-top: 10px;
    color: var(--muted);
    line-height: 1.5;
    font-size: 0.82rem;
}

.main-panel { padding: 28px; }
.surface {
    position: relative;
    min-height: calc(100vh - 56px);
    padding: 28px;
    border-radius: var(--radius-xl);
    background: linear-gradient(180deg, rgba(255, 255, 255, 0.82), rgba(246, 249, 255, 0.76));
    border: 1px solid rgba(114, 135, 201, 0.14);
    box-shadow: var(--shadow), inset 0 1px 0 rgba(255,255,255,0.6);
    backdrop-filter: blur(24px);
}
.surface::before {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: inherit;
    background:
        radial-gradient(circle at 20% 0%, rgba(143, 109, 255, 0.12), transparent 24%),
        radial-gradient(circle at 100% 20%, rgba(99, 164, 255, 0.1), transparent 24%);
    pointer-events: none;
}

.topbar {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 18px;
    margin-bottom: 28px;
}
.topbar-eyebrow {
    margin: 0 0 8px;
    color: var(--cyan);
    text-transform: uppercase;
    letter-spacing: 0.18em;
    font-size: 0.75rem;
}
.topbar-title {
    margin: 0;
    font-size: var(--type-display);
    letter-spacing: -0.05em;
    line-height: 0.95;
}
.topbar-copy {
    margin: 14px 0 0;
    max-width: 620px;
    color: var(--muted);
    line-height: 1.6;
    font-size: var(--type-body);
}
.topbar-side { display: flex; align-items: center; gap: 12px; flex-wrap: wrap; }

.chip {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    border-radius: 999px;
    background: rgba(255,255,255,0.64);
    border: 1px solid rgba(114, 135, 201, 0.12);
    color: var(--muted);
    font-size: 0.84rem;
}
.chip strong { color: var(--text); font-weight: 600; }
.chip.demo {
    background: rgba(255, 193, 107, 0.16);
    border-color: rgba(255, 193, 107, 0.24);
    color: #9f6a00;
}

.content-grid { display: grid; gap: 20px; }
.dashboard-grid { grid-template-columns: 1.4fr 0.9fr; align-items: start; }
.stack { display: grid; gap: 20px; }

.hero-card, .panel-card {
    position: relative;
    overflow: hidden;
    border-radius: var(--radius-lg);
    padding: var(--space-4);
    background: linear-gradient(180deg, rgba(255, 255, 255, 0.82), rgba(246, 249, 255, 0.78));
    border: 1px solid rgba(114, 135, 201, 0.14);
    box-shadow: var(--shadow-soft);
    transition: transform 220ms ease, border-color 220ms ease, box-shadow 220ms ease;
}
.hero-card:hover, .panel-card:hover {
    transform: translateY(-4px);
    border-color: rgba(143, 109, 255, 0.24);
    box-shadow: 0 24px 70px rgba(0, 0, 0, 0.34);
}
.hero-card::before, .panel-card::before {
    content: "";
    position: absolute;
    inset: auto -12% -50% auto;
    width: 220px;
    height: 220px;
    border-radius: 999px;
    background: radial-gradient(circle, rgba(99, 164, 255, 0.24), transparent 65%);
    pointer-events: none;
}
.hero-card::after {
    content: "";
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg, rgba(143,109,255,0.18), transparent 40%, rgba(255,107,122,0.1));
    pointer-events: none;
}

.section-title { margin: 0 0 10px; font-size: 1.1rem; font-weight: 600; }
.section-copy { margin: 0; color: var(--muted); line-height: 1.6; }
.wallet-balance {
    margin: 18px 0 10px;
    font-size: clamp(2.4rem, 5vw, 4rem);
    line-height: 1;
    letter-spacing: -0.05em;
}
.wallet-subline {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    align-items: center;
    color: var(--muted);
}
.address-pill {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.62);
    border: 1px solid rgba(114, 135, 201, 0.12);
    font-family: "SFMono-Regular", Consolas, monospace;
    font-size: 0.84rem;
}

.button-row { display: flex; flex-wrap: wrap; gap: 12px; margin-top: 24px; }
.primary-button, .ghost-button, .copy-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    min-height: 48px;
    padding: 0 18px;
    border-radius: var(--radius-md);
    border: 1px solid transparent;
    cursor: pointer;
    transition: transform 180ms ease, box-shadow 180ms ease, border-color 180ms ease, background 180ms ease, opacity 180ms ease;
}
.primary-button {
    color: white;
    background: linear-gradient(135deg, var(--purple), var(--blue));
    box-shadow: 0 18px 34px rgba(111, 94, 255, 0.32);
}
.primary-button:hover { transform: translateY(-2px); box-shadow: 0 24px 38px rgba(111, 94, 255, 0.42); }
.primary-button:disabled, .primary-button.disabled { opacity: 0.5; cursor: not-allowed; box-shadow: none; }
.ghost-button, .copy-button {
    color: var(--text);
    background: rgba(255,255,255,0.62);
    border-color: rgba(114, 135, 201, 0.14);
}
.ghost-button:hover, .copy-button:hover {
    transform: translateY(-2px);
    border-color: rgba(99,164,255,0.28);
    background: rgba(255,255,255,0.82);
}

.stats-grid, .settings-grid, .send-grid, .receive-grid { display: grid; gap: 20px; }
.stats-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); }
.settings-grid, .send-grid, .receive-grid { grid-template-columns: 1.25fr 0.9fr; }

.metric { display: flex; flex-direction: column; gap: 10px; }
.metric-label, .eyebrow {
    color: var(--muted);
    text-transform: uppercase;
    letter-spacing: 0.16em;
    font-size: 0.75rem;
}
.metric-value { font-size: 2rem; line-height: 1; letter-spacing: -0.04em; }
.metric-change { color: var(--success); font-size: 0.84rem; }

.status-list, .summary-list, .detail-list { display: grid; gap: 14px; }
.status-item, .summary-item, .detail-item, .activity-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 14px 0;
    border-bottom: 1px solid rgba(255,255,255,0.06);
}
.status-item:last-child, .summary-item:last-child, .detail-item:last-child, .activity-item:last-child { border-bottom: none; }
.status-item label, .summary-item label, .detail-item label { color: var(--muted); }
.status-good { color: var(--success); }
.status-warn { color: #ffd47a; }

.activity-list { display: grid; gap: 10px; }
.activity-item {
    padding: 16px 18px;
    border-radius: 18px;
    background: rgba(255,255,255,0.56);
    border: 1px solid rgba(114, 135, 201, 0.12);
}
.activity-copy { display: grid; gap: 4px; }
.activity-title { font-weight: 600; }
.activity-meta { color: var(--muted); font-size: 0.82rem; }
.activity-amount { text-align: right; }
.activity-amount strong { display: block; font-size: 1rem; }
.activity-amount span { color: var(--muted); font-size: 0.82rem; }
.pill {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 84px;
    padding: 6px 10px;
    border-radius: 999px;
    font-size: 0.72rem;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    border: 1px solid rgba(255,255,255,0.08);
}
.pill.incoming { color: var(--success); background: rgba(121,247,196,0.08); }
.pill.outgoing { color: #ff9aa6; background: rgba(255,107,122,0.08); }
.pill.pending { color: #ffd47a; background: rgba(255,212,122,0.08); }

.field-grid { display: grid; gap: 16px; }
.field-group { display: grid; gap: 8px; }
.field-label { color: var(--muted); font-size: 0.86rem; }
.field-input, .field-select {
    width: 100%;
    min-height: 54px;
    padding: 0 16px;
    border-radius: 16px;
    border: 1px solid rgba(114, 135, 201, 0.14);
    background: rgba(255, 255, 255, 0.68);
    color: var(--text);
    outline: none;
    transition: border-color 180ms ease, box-shadow 180ms ease, transform 180ms ease;
}
.field-input:focus, .field-select:focus {
    border-color: rgba(99,164,255,0.55);
    box-shadow: 0 0 0 4px rgba(99,164,255,0.14);
    transform: translateY(-1px);
}
.field-error {
    border-color: rgba(255,107,122,0.52);
    box-shadow: 0 0 0 4px rgba(255,107,122,0.12);
}
.field-help { color: var(--soft); font-size: 0.8rem; }
.field-help.error { color: #ff9aa6; }
.two-up { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 16px; }

.summary-highlight {
    margin-top: 16px;
    padding: 16px;
    border-radius: var(--radius-md);
    background: linear-gradient(135deg, rgba(143,109,255,0.12), rgba(99,164,255,0.08));
    border: 1px solid rgba(143,109,255,0.18);
}
.summary-highlight strong { display: block; margin-bottom: 6px; font-size: 0.95rem; }
.summary-highlight p { margin: 0; color: var(--muted); line-height: 1.5; }

.receive-address {
    padding: 18px;
    border-radius: 20px;
    background: rgba(255, 255, 255, 0.66);
    border: 1px solid rgba(114, 135, 201, 0.14);
    font-family: "SFMono-Regular", Consolas, monospace;
    line-height: 1.7;
    word-break: break-all;
    color: #233058;
}
.qr-shell {
    width: min(100%, 320px);
    aspect-ratio: 1 / 1;
    padding: 18px;
    border-radius: 28px;
    background: linear-gradient(180deg, rgba(255,255,255,0.8), rgba(245,248,255,0.72));
    border: 1px solid rgba(114, 135, 201, 0.14);
    box-shadow: 0 24px 50px rgba(92, 121, 209, 0.16);
}
.qr-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 8px;
    width: 100%;
    height: 100%;
}
.qr-grid span { border-radius: var(--space-1); background: rgba(255,255,255,0.08); }
.qr-grid span.on {
    background: linear-gradient(135deg, rgba(143,109,255,0.95), rgba(99,164,255,0.95));
    box-shadow: 0 0 18px rgba(99,164,255,0.2);
}

.message {
    margin-top: 18px;
    padding: 14px 16px;
    border-radius: 16px;
    border: 1px solid rgba(114, 135, 201, 0.14);
    background: rgba(255,255,255,0.58);
    color: var(--muted);
}
.message.success { color: var(--success); border-color: rgba(121,247,196,0.22); background: rgba(121,247,196,0.08); }
.message.warn { color: #ffd47a; border-color: rgba(255,212,122,0.18); background: rgba(255,212,122,0.08); }
.message.error { color: #ff9aa6; border-color: rgba(255,107,122,0.22); background: rgba(255,107,122,0.08); }
.settings-note { margin-top: 18px; color: var(--muted); line-height: 1.6; }
.trust-note {
    margin-top: 16px;
    padding: 16px;
    border-radius: var(--radius-md);
    background: rgba(255, 193, 107, 0.1);
    border: 1px solid rgba(255, 193, 107, 0.22);
    color: #7c5900;
    line-height: 1.6;
}

.not-found { min-height: 60vh; display: grid; place-items: center; }
.not-found-card {
    width: min(100%, 620px);
    padding: 32px;
    border-radius: 28px;
    text-align: center;
    background: linear-gradient(180deg, rgba(20, 26, 42, 0.95), rgba(12, 16, 28, 0.92));
    border: 1px solid rgba(255,255,255,0.08);
}

@keyframes floatGlow {
    0%, 100% { transform: translate3d(0, 0, 0) scale(1); }
    50% { transform: translate3d(24px, 18px, 0) scale(1.08); }
}

@media (max-width: 1120px) {
    .app-frame, .dashboard-grid, .send-grid, .receive-grid, .settings-grid { grid-template-columns: 1fr; }
    .sidebar {
        padding-bottom: 0;
        border-right: none;
        border-bottom: 1px solid rgba(255,255,255,0.06);
    }
}

@media (max-width: 780px) {
    .main-panel, .surface { padding: 18px; }
    .topbar { flex-direction: column; }
    .stats-grid, .two-up { grid-template-columns: 1fr; }
    .topbar-title { font-size: clamp(2rem, 14vw, 3rem); }
    .button-row > * { width: 100%; }
    .address-pill, .chip { width: 100%; justify-content: center; }
}

@media (prefers-reduced-motion: reduce) {
    *,
    *::before,
    *::after {
        animation: none !important;
        transition: none !important;
    }
}
"#;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub rpc_url: String,
    pub explorer_url: String,
    pub environment: String,
    pub network_label: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            rpc_url: "https://rpc.xorion.network".to_string(),
            explorer_url: "https://scan.xorion.network".to_string(),
            environment: "MVP Demo".to_string(),
            network_label: "Xorion Testnet".to_string(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ActivityItem {
    pub title: String,
    pub subtitle: String,
    pub amount: String,
    pub state: String,
    pub direction: String,
}

#[derive(Clone, PartialEq)]
pub struct WalletSnapshot {
    pub wallet_name: String,
    pub session_label: String,
    pub short_address: String,
    pub full_address: String,
    pub primary_balance: String,
    pub secondary_balance: String,
    pub change_label: String,
    pub rpc_state: String,
    pub network_health: String,
    pub synced_at: String,
    pub chain_name: String,
    pub chain_id: String,
    pub latest_block: String,
    pub is_live: bool,
    pub is_syncing: bool,
    pub status_message: String,
    pub activity: Vec<ActivityItem>,
}

impl Default for WalletSnapshot {
    fn default() -> Self {
        Self {
            wallet_name: "Xorion Prime Wallet".to_string(),
            session_label: "Disconnected".to_string(),
            short_address: "Not connected".to_string(),
            full_address: "Connect a browser wallet or import an address to start the MVP session.".to_string(),
            primary_balance: "0.0000 ETH".to_string(),
            secondary_balance: "--".to_string(),
            change_label: "Awaiting wallet session".to_string(),
            rpc_state: "Checking".to_string(),
            network_health: "Unknown".to_string(),
            synced_at: "Not synced".to_string(),
            chain_name: "Unknown".to_string(),
            chain_id: "--".to_string(),
            latest_block: "--".to_string(),
            is_live: false,
            is_syncing: false,
            status_message: "Connect a browser wallet or import a watch-only address.".to_string(),
            activity: Vec::new(),
        }
    }
}

impl WalletSnapshot {
    pub fn syncing(session: &WalletSession) -> Self {
        let mut snapshot = Self::from_state(session, None, None);
        snapshot.rpc_state = "Syncing".to_string();
        snapshot.network_health = "Checking".to_string();
        snapshot.synced_at = "Refreshing...".to_string();
        snapshot.is_syncing = true;
        snapshot.status_message = if session.address.is_some() {
            "Refreshing live wallet data from the configured EVM RPC.".to_string()
        } else {
            "Checking the configured EVM RPC. Connect or import an address for live account reads.".to_string()
        };
        snapshot
    }

    pub fn from_state(session: &WalletSession, overview: Option<RpcOverview>, error: Option<String>) -> Self {
        let mut snapshot = Self::default();
        snapshot.wallet_name = if session.wallet_name.is_empty() {
            "Xorion Wallet".to_string()
        } else {
            session.wallet_name.clone()
        };
        snapshot.session_label = match session.mode {
            crate::session::SessionMode::Disconnected => "Disconnected".to_string(),
            crate::session::SessionMode::BrowserWallet => "Browser Wallet".to_string(),
            crate::session::SessionMode::ImportedAddress => "Imported Address".to_string(),
            crate::session::SessionMode::PreviewWallet => "Preview Wallet".to_string(),
        };
        snapshot.short_address = session.short_address();
        snapshot.full_address = session
            .address
            .clone()
            .unwrap_or_else(|| "Connect a browser wallet or import an address to start the MVP session.".to_string());
        snapshot.status_message = session.status_message.clone();

        if let Some(overview) = overview {
            snapshot.primary_balance = overview.balance_display.clone();
            snapshot.secondary_balance = format!("Chain {} · Block {}", overview.chain_name, overview.latest_block);
            snapshot.change_label = "Live RPC data".to_string();
            snapshot.rpc_state = overview.rpc_status;
            snapshot.network_health = overview.network_health;
            snapshot.synced_at = overview.synced_at;
            snapshot.chain_name = overview.chain_name;
            snapshot.chain_id = overview.chain_id;
            snapshot.latest_block = overview.latest_block;
            snapshot.is_live = session.address.is_some();
            snapshot.is_syncing = false;
        } else if let Some(error) = error {
            snapshot.rpc_state = "Unavailable".to_string();
            snapshot.network_health = "Degraded".to_string();
            snapshot.change_label = "RPC unavailable".to_string();
            snapshot.status_message = error;
            snapshot.is_syncing = false;
        }

        snapshot
    }
}

pub fn load_settings() -> Option<AppSettings> {
    #[cfg(target_arch = "wasm32")]
    {
        let storage = web_sys::window()
            .and_then(|window| window.local_storage().ok().flatten())?;
        let raw = storage.get_item("xorion.webapp.settings").ok().flatten()?;
        serde_json::from_str(&raw).ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

pub fn save_settings(settings: &AppSettings) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let storage = web_sys::window()
            .and_then(|window| window.local_storage().ok().flatten())
            .ok_or_else(|| "Browser storage is unavailable".to_string())?;
        let payload = serde_json::to_string(settings)
            .map_err(|err| format!("Could not serialize settings: {err}"))?;
        storage
            .set_item("xorion.webapp.settings", &payload)
            .map_err(|_| "Could not save settings in browser storage".to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = settings;
        Ok(())
    }
}

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
    let wallet = use_context::<WalletStore>();

    rsx! {
        div { class: "app-shell",
            style { "{APP_CSS}" }
            div { class: "ambient" }
            div { class: "app-frame",
                aside { class: "sidebar",
                    div { class: "brand-lockup",
                        div { class: "brand-pill",
                            div { class: "brand-mark", "X" }
                            div { class: "brand-copy",
                                h1 { "Xorion Wallet" }
                                p { "Quantum-grade digital assets" }
                            }
                        }
                    }
                    p { class: "sidebar-blurb",
                        "A premium Web3 control surface for balances, transfers, and infrastructure settings."
                    }
                    nav { class: "nav-stack",
                        Link {
                            class: "nav-link",
                            active_class: "active",
                            to: Route::Dashboard {},
                            div { class: "nav-meta", span { class: "nav-title", "Dashboard" } span { class: "nav-subtitle", "Portfolio overview" } }
                            div { class: "nav-badge", "01" }
                        }
                        Link {
                            class: "nav-link",
                            active_class: "active",
                            to: Route::Send {},
                            div { class: "nav-meta", span { class: "nav-title", "Send" } span { class: "nav-subtitle", "Move capital out" } }
                            div { class: "nav-badge", "02" }
                        }
                        Link {
                            class: "nav-link",
                            active_class: "active",
                            to: Route::Receive {},
                            div { class: "nav-meta", span { class: "nav-title", "Receive" } span { class: "nav-subtitle", "Fund your wallet" } }
                            div { class: "nav-badge", "03" }
                        }
                        Link {
                            class: "nav-link",
                            active_class: "active",
                            to: Route::Settings {},
                            div { class: "nav-meta", span { class: "nav-title", "Settings" } span { class: "nav-subtitle", "RPC and explorer" } }
                            div { class: "nav-badge", "04" }
                        }
                    }
                    div { class: "sidebar-status",
                        div { class: "status-row", div { class: "status-dot" } strong { "{wallet.read().rpc_state}" } }
                        p { class: "status-copy",
                            "{wallet.read().session_label} session. {wallet.read().synced_at}. {wallet.read().status_message}"
                        }
                    }
                }
                main { class: "main-panel",
                    div { class: "surface", Outlet::<Route> {} }
                }
            }
        }
    }
}

#[component]
fn Dashboard() -> Element {
    rsx! { DashboardScreen {} }
}

#[component]
fn Send() -> Element {
    rsx! { SendScreen {} }
}

#[component]
fn Receive() -> Element {
    rsx! { ReceiveScreen {} }
}

#[component]
fn Settings() -> Element {
    rsx! { SettingsScreen {} }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "not-found",
            div { class: "not-found-card",
                p { class: "topbar-eyebrow", "Route unavailable" }
                h1 { class: "topbar-title", "404" }
                p { class: "topbar-copy",
                    "The requested wallet route could not be found. Missing path: /{route.join(\"/\")}"
                }
                div { class: "button-row", Link { class: "primary-button", to: Route::Dashboard {}, "Return to Dashboard" } }
            }
        }
    }
}

#[component]
pub fn App() -> Element {
    let settings = use_signal(|| load_settings().unwrap_or_default());
    let session = use_signal(|| load_session().unwrap_or_else(WalletSession::disconnected));
    let mut wallet = use_signal(WalletSnapshot::default);
    let refresh_nonce = use_signal(|| 0_u64);
    use_context_provider(|| settings);
    use_context_provider(|| session);
    use_context_provider(|| wallet);
    use_context_provider(|| refresh_nonce);

    let _ = use_resource(move || {
        let rpc_url = settings.read().rpc_url.clone();
        let session_value = session.read().clone();
        let _refresh = refresh_nonce();

        async move {
            wallet.set(WalletSnapshot::syncing(&session_value));
            match fetch_overview(&rpc_url, session_value.address.as_deref()).await {
                Ok(overview) => wallet.set(WalletSnapshot::from_state(&session_value, Some(overview), None)),
                Err(error) => wallet.set(WalletSnapshot::from_state(&session_value, None, Some(error))),
            }
        }
    });

    rsx! { Router::<Route> {} }
}
