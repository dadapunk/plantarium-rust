use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::Router;

mod app_state;
mod components;
mod pages;
mod router;

fn main() {
    app_state::load_from_storage();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }
        Router::<Route> {}
    }
}
