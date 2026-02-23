use dioxus::prelude::*;

#[component]
pub fn FilePreview(id: i64, type_name: String) -> Element {
    rsx! {
        div {
            class: "file-preview",
            match type_name.as_str() {
                "images" => rsx! { img { class: "img-preview", src: "/api/media/{id}" } },
                _ => rsx! {p { "No preview" } }
            }
        }
    }
}
