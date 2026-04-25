mod models;
mod api;
mod auth;
mod router;
mod components;
mod pdf;

use components::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}