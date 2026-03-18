use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::Router;

mod app_state;
mod components;
mod pages;
mod router;
mod storage;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    app_state::load_from_storage();

    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }
        Router::<Route> {}
    }
}
