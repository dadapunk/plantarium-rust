use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::Router;

mod app_state;
mod components;
mod layouts;
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
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Noto+Serif:ital,wght@0,400;0,700;1,400&family=Manrope:wght@400;500;600;700&display=swap" }
        document::Stylesheet { href: asset!("/assets/plantarium-theme.css") }
        document::Stylesheet { href: asset!("/assets/main.css") }
        document::Stylesheet { href: asset!("/assets/tasks.css") }
        body {
            Router::<Route> {}
        }
    }
}
