use dioxus::prelude::*;

use crate::components::filters_bar::FiltersBar;
use crate::components::header::Header;
use crate::components::search_bar::SearchBar;

#[component]
pub fn Home() -> Element {
    let pattern = use_signal(|| String::new());
    let filters = use_signal(|| vec![true; 5]);

    rsx! {
        div {
            class: "home",
            Header {}
            FiltersBar {
                filters: filters
            }
            SearchBar {
                pattern: pattern,
                on_submit: move |_| {
                    tracing::debug!("ENTERED : {:?} {:?}", pattern(), filters())
                }
            }
        }
    }
}
