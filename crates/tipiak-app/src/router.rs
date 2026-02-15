use dioxus::prelude::*;

use crate::routes::home::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
}
