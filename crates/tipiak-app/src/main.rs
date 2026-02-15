mod api;
mod app;
mod components;
mod config;
mod router;
mod routes;

use crate::app::App;

fn main() {
    dioxus::launch(App);
}
