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
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use crate::api::media::download::media_download;
        use dioxus::server::axum::routing::get; // ← chemin correct

        let router =
            dioxus::server::router(App).route("/api/media/download/{id}", get(media_download));

        Ok(router)
    });
}
