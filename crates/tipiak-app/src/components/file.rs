use dioxus::prelude::*;

use crate::api::search::PayloadFile;
use crate::components::file_preview::FilePreview;
use crate::utils::icon_for_type;

#[component]
pub fn File(file: PayloadFile) -> Element {
    rsx! {
        div {
            class: "file",
            div {
                class: "file-header",
                img {
                    src: icon_for_type(&file.type_name),
                    width: 20,
                    height: 20
                }
                p { "{file.path}" }
                a {
                    class: "download-link",
                    href: "/api/media/download/{file.id}",
                    download: true,
                    rel: "external",
                    "Download"
                }
            }
            FilePreview { id: file.id, type_name: file.type_name.clone() }
        }
    }
}
