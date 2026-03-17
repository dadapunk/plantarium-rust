use crate::app_state::{create_garden, GARDENS};
use crate::components::Navbar;
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    let mut new_garden_name = use_signal(|| String::new());
    let gardens = GARDENS.read();

    let mut add_garden = move || {
        let name = new_garden_name();
        if !name.trim().is_empty() {
            create_garden(&name);
            new_garden_name.set(String::new());
        }
    };

    rsx! {
        div { class: "app-container",
            Navbar {}
            div { class: "main-content",
                header { class: "dashboard-header",
                    h1 { "Jardines" }
                }

                div { class: "add-form",
                    input {
                        r#type: "text",
                        placeholder: "Nombre del jardín...",
                        value: "{new_garden_name}",
                        oninput: move |evt| new_garden_name.set(evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter {
                                add_garden();
                            }
                        }
                    }
                    button { onclick: move |_| add_garden(), "+ Añadir Jardín" }
                }

                if gardens.is_empty() {
                    div { class: "empty",
                        p { "No hay jardines todavía" }
                        p { "¡Crea tu primer jardín!" }
                    }
                } else {
                    div { class: "gardens-grid",
                        for garden in gardens.iter() {
                            div { class: "garden-card",
                                Link {
                                    to: Route::GardenDetail { id: garden.base.id.clone() },
                                    h2 { "{garden.name}" }
                                }
                                p { "{crate::app_state::get_garden_beds(&garden.base.id).len()} bancales" }
                            }
                        }
                    }
                }
            }
        }
    }
}
