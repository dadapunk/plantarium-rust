use crate::components::Navbar;
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "app-container",
            Navbar {}
            Outlet::<Route> {}
        }
    }
}
