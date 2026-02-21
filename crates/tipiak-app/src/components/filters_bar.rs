use dioxus::prelude::*;

use crate::api::config::file_types;

const IMAGES_ICON: Asset = asset!("/assets/images.png");
const VIDEOS_ICON: Asset = asset!("/assets/videos.png");
const SOUNDS_ICON: Asset = asset!("/assets/sounds.png");
const TEXT_ICON: Asset = asset!("/assets/text.png");
const WEB_ICON: Asset = asset!("/assets/web.png");
const PLACEHOLDER_ICON: Asset = asset!("/assets/placeholder.png");

#[derive(Props, Clone, PartialEq)]
pub struct FiltersBarProps {
    filters: Signal<Vec<bool>>,
}

fn icon_for_type(type_name: &str) -> Asset {
    match type_name {
        "images" => IMAGES_ICON,
        "videos" => VIDEOS_ICON,
        "sounds" => SOUNDS_ICON,
        "text" => TEXT_ICON,
        "web" => WEB_ICON,
        _ => PLACEHOLDER_ICON,
    }
}

#[component]
pub fn FiltersBar(mut props: FiltersBarProps) -> Element {
    let file_types_resource = use_resource(|| async { file_types().await });

    use_effect(move || {
        if let Some(Ok(map)) = &*file_types_resource.read() {
            let len = map.len();
            if len != props.filters.read().len() {
                props.filters.set(vec![true; len]);
            }
        }
    });

    rsx! {
        div {
            class: "filters-bar",

            match &*file_types_resource.read() {
                Some(Ok(map)) => rsx! {
                    for (i, (type_name, _extensions)) in map.iter().enumerate() {
                        button {
                            class: if (props.filters)().get(i).copied().unwrap_or(false) {
                                "filters-button filters-button-active"
                            } else {
                                "filters-button"
                            },
                            title: "{type_name}",
                            onclick: move |_| {
                                props.filters.with_mut(|s| {
                                    if i < s.len() {
                                        s[i] = !s[i];
                                    }
                                });
                            },
                            img {
                                width: 20,
                                src: icon_for_type(type_name),
                            }
                        }
                    }
                },
                Some(Err(_)) => rsx! {
                    div { "Failed to load filters" }
                },
                None => rsx! {
                    div { "Loading filters..." }
                }
            }
        }
    }
}
