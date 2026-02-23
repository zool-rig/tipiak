use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/logo-32x32.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");

use crate::components::font::FontFace;
use crate::router::Route;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        FontFace { family: "PublicPixel", asset: asset!("/assets/PublicPixel-eZPz6.woff") }
        Router::<Route> {}
    }
}
