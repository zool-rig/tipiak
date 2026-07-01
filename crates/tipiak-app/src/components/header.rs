use dioxus::prelude::*;

use crate::utils::get_main_icon;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "header",
            img { class: "logo", src: get_main_icon() }
            h1 { class: "title", "TIPIAK" }
        }
    }
}
