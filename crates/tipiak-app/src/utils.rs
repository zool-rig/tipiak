use dioxus::prelude::*;

const IMAGES_ICON: Asset = asset!("/assets/images.png");
const VIDEOS_ICON: Asset = asset!("/assets/videos.png");
const SOUNDS_ICON: Asset = asset!("/assets/sounds.png");
const TEXT_ICON: Asset = asset!("/assets/text.png");
const WEB_ICON: Asset = asset!("/assets/web.png");
const PLACEHOLDER_ICON: Asset = asset!("/assets/placeholder.png");
const MAIN_ICON_64: Asset = asset!("/assets/logo-64x64.png");

pub fn encode_filters(filters: &[bool]) -> String {
    filters.iter().map(|b| if *b { '1' } else { '0' }).collect()
}

pub fn decode_filters(s: &str) -> Vec<bool> {
    s.chars().map(|c| c == '1').collect()
}

pub fn icon_for_type(type_name: &str) -> Asset {
    match type_name {
        "images" => IMAGES_ICON,
        "videos" => VIDEOS_ICON,
        "sounds" => SOUNDS_ICON,
        "text" => TEXT_ICON,
        "web" => WEB_ICON,
        _ => PLACEHOLDER_ICON,
    }
}

pub fn get_main_icon() -> Asset {
    MAIN_ICON_64
}
