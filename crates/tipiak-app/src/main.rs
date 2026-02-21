mod api;
mod app;
mod components;
mod config;
mod constants;
mod router;
mod routes;
mod utils;

use crate::app::App;

fn main() {
    dioxus::launch(App);
}
