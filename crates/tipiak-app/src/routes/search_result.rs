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
    let pattern_signal = use_signal(|| pattern.clone());
    let filters_signal = use_signal(|| decode_filters(&filters));

    let mut submitted_pattern = use_signal(|| pattern.clone());
    let mut submitted_filters = use_signal(|| decode_filters(&filters));

    let matching_files = use_resource(move || {
        let pattern = submitted_pattern();
        let filters = submitted_filters();
        async move { search(pattern.trim().to_string(), filters).await }
    });

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
                    submitted_pattern.set(pattern_signal().trim().to_string());
                    submitted_filters.set(filters_signal());
                    navigator.replace(
                        Route::SearchResult {
                            pattern: submitted_pattern().trim().to_string(),
                            filters: encode_filters(&submitted_filters())
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
                Some(Err(_)) => rsx! {
                    div { "Failed to load files" }
                },
                None => rsx! { "Loading files..." }
            }
        }
    }
}
