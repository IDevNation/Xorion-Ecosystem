mod app;
mod components;
mod explorer;
mod rpc;
mod session;

use dioxus::prelude::*;

fn main() {
    launch(app::App);
}
