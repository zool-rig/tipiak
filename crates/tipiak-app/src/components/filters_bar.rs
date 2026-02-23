use dioxus::prelude::*;

use crate::api::config::file_types;
use crate::utils::icon_for_type;

#[derive(Props, Clone, PartialEq)]
pub struct FiltersBarProps {
    filters: Signal<Vec<bool>>,
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
                    for (i, type_name) in map.iter().enumerate() {
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
