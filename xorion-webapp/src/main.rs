use dioxus::prelude::*;

mod app;
mod components;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(app::App);
}
