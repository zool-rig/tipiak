use dioxus::prelude::*;

const IMAGES_ICON: Asset = asset!("/assets/images.png");
const VIDEOS_ICON: Asset = asset!("/assets/videos.png");
const SOUNDS_ICON: Asset = asset!("/assets/sounds.png");
const TEXT_ICON: Asset = asset!("/assets/text.png");
const WEB_ICON: Asset = asset!("/assets/web.png");

const ICONS: [Asset; 5] = [IMAGES_ICON, VIDEOS_ICON, SOUNDS_ICON, TEXT_ICON, WEB_ICON];

#[derive(Props, Clone, PartialEq)]
pub struct FiltersBarProps {
    filters: Signal<Vec<bool>>,
}

#[component]
pub fn FiltersBar(mut props: FiltersBarProps) -> Element {
    rsx! {
        div {
            class: "filters-bar",
            for (i, icon) in ICONS.iter().enumerate() {
                button {
                    class: if (props.filters)()[i] {
                        "filters-button filters-button-active"
                    } else {
                        "filters-button"
                    },
                    title: "toto",
                    onclick: move |_| {
                        props.filters.with_mut(|s| {
                            s[i] = !s[i];
                        });
                    },
                    img {
                        width: 20,
                        src: *icon,
                    }
                }
            }
        }
    }
}
