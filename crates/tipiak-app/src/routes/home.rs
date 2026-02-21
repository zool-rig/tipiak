use dioxus::prelude::*;

use crate::components::filters_bar::FiltersBar;
use crate::components::header::Header;
use crate::components::search_bar::SearchBar;
use crate::router::Route;
use crate::utils::encode_filters;

#[component]
pub fn Home() -> Element {
    let pattern = use_signal(|| String::new());
    let filters = use_signal(|| vec![true; 5]);
    let navigator = use_navigator();

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
                    navigator.push(
                        Route::SearchResult {
                            pattern: pattern().trim().to_string(),
                            filters: encode_filters(&filters())
                        }
                    );
                }
            }
        }
    }
}
