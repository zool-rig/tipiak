use dioxus::prelude::*;

use crate::routes::home::Home;
use crate::routes::search_result::SearchResult;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/search/:pattern/:filters")]
    SearchResult { 
        pattern: String,
        filters: String,
    },
}
