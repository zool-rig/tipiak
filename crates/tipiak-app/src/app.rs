use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/logo-32x32.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");

use crate::router::Route;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: "https://fonts.cdnfonts.com/css/public-pixel"}
        Router::<Route> {}
    }
}
