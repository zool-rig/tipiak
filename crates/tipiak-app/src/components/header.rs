use dioxus::prelude::*;

const ICON: Asset = asset!("/assets/logo-64x64.png");

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "header",
            img { class: "logo", src: ICON }
            h1 { class: "title", "TIPIAK" }
        }
    }
}
