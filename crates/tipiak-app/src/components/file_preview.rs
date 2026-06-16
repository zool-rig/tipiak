use dioxus::prelude::*;
use gloo_net::http::Request;

#[component]
pub fn FilePreview(id: i64, type_name: String) -> Element {
    let type_name_clone = type_name.clone();
    let text_content = use_resource(move || {
        let id = id;
        let type_name = type_name_clone.clone();

        async move {
            if type_name.as_str() == "text" {
                let resp = Request::get(&format!("/api/media/{id}"))
                    .send()
                    .await
                    .ok()?
                    .text()
                    .await
                    .ok()?;

                Some(resp)
            } else {
                None
            }
        }
    });

    rsx! {
        div {
            class: "file-preview",
            match type_name.as_str() {
                "images" => rsx! { img { class: "img-preview", src: "/api/media/{id}" } },
                "text" => rsx! {
                    match &*text_content.read() {
                        Some(Some(content)) => rsx! {
                            pre {
                                class: "text-preview",
                                code { "{content}" }
                            }
                        },
                        Some(None) => rsx! { p { "Failed to read file" } },
                        None => rsx! { p { "Loading..." } }
                    }
                },
                "sounds" => rsx! {
                    audio {
                        class: "sound-preview",
                        src: "/api/media/{id}",
                        controls: true,
                    }
                },
                "videos" => rsx! {
                    video {
                        src: "/api/media/{id}",
                        controls: true,
                    }
                },
                _ => rsx! {p { "No preview" } }
            }
        }
    }
}
