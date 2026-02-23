use dioxus::prelude::*;

use crate::api::search::search;
use crate::components::file::File;
use crate::components::filters_bar::FiltersBar;
use crate::components::header::Header;
use crate::components::search_bar::SearchBar;
use crate::router::Route;
use crate::utils::{decode_filters, encode_filters};

#[component]
pub fn SearchResult(pattern: String, filters: String) -> Element {
    let pattern_clone = pattern.clone();
    let filters_clone = filters.clone();
    let matching_files = use_resource(move || {
        let pattern = pattern_clone.clone();
        let filters = filters_clone.clone();
        async move { search(pattern, decode_filters(&filters)).await }
    });
    let pattern_signal = use_signal(|| pattern);
    let filters_signal = use_signal(|| decode_filters(&filters));
    let navigator = use_navigator();

    rsx! {
        div {
            class: "search-result",
            Header {}
            FiltersBar {
                filters: filters_signal
            }
            SearchBar {
                pattern: pattern_signal,
                on_submit: move |_| {
                    navigator.push(
                        Route::SearchResult {
                            pattern: pattern_signal().trim().to_string(),
                            filters: encode_filters(&filters_signal())
                        }
                    );
                }
            },

            match &*matching_files.read() {
                Some(Ok(files)) => {
                    rsx! {
                        div { class: "separator" }
                        div {
                            class: "search-result-viewport",
                            for file in files {
                                File { file: file.clone() }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { "Failed to load files {e}" }
                },
                None => rsx! { "Loading files..." }
            }
        }
    }
}
