use dioxus::prelude::*;

#[component]
pub fn FontFace(family: &'static str, asset: Asset) -> Element {
    rsx! {
        document::Style {{
            format!("
                @font-face {{
                    font-family: '{}';
                    src: url('{}') format('woff');
                }}
                ", family, asset
            )
        }}
    }
}
