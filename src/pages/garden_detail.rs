use crate::app_state::{create_bed, delete_bed, BEDS, GARDENS};
use crate::components::Navbar;
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn GardenDetail(id: String) -> Element {
    let mut new_bed_name = use_signal(|| String::new());
    let mut new_bed_width = use_signal(|| 200i32);
    let mut new_bed_height = use_signal(|| 100i32);

    let garden = GARDENS.read().iter().find(|g| g.base.id == id).cloned();
    let beds = BEDS
        .read()
        .iter()
        .filter(|b| b.garden_id == id)
        .cloned()
        .collect::<Vec<_>>();

    let beds_list: Vec<_> = beds
        .iter()
        .map(|b| {
            (
                b.base.id.clone(),
                b.name.clone(),
                b.width,
                b.height,
                b.plants.len(),
            )
        })
        .collect();

    let id_clone = id.clone();
    let mut add_bed = move || {
        let name = new_bed_name();
        if !name.trim().is_empty() {
            create_bed(&id_clone, &name, new_bed_width(), new_bed_height());
            new_bed_name.set(String::new());
        }
    };

    rsx! {
        div { class: "app-container",
            Navbar {}
            div { class: "main-content",
                div { class: "header",
                    Link { to: Route::Dashboard {}, "← Volver" }
                    if let Some(g) = &garden {
                        h1 { "{g.name}" }
                    }
                }

                div { class: "add-form",
                    input {
                        r#type: "text",
                        placeholder: "Nombre del bancal",
                        value: "{new_bed_name}",
                        oninput: move |evt| new_bed_name.set(evt.value()),
                    }
                    input {
                        r#type: "number",
                        placeholder: "Ancho (cm)",
                        value: "{new_bed_width}",
                        oninput: move |evt| new_bed_width.set(evt.value().parse().unwrap_or(200)),
                    }
                    input {
                        r#type: "number",
                        placeholder: "Alto (cm)",
                        value: "{new_bed_height}",
                        oninput: move |evt| new_bed_height.set(evt.value().parse().unwrap_or(100)),
                    }
                    button { onclick: move |_| add_bed(), "+ Añadir Bancal" }
                }

                if beds.is_empty() {
                    div { class: "empty",
                        p { "No hay bancales" }
                        p { "¡Crea tu primer bancal!" }
                    }
                } else {
                    div { class: "beds-grid",
                        for (bed_id, bed_name, bed_width, bed_height, plants_len) in beds_list {
                            div { class: "bed-card",
                                Link {
                                    to: Route::BedEditor { id: bed_id.clone() },
                                    div { class: "bed-preview",
                                        style: "width: {bed_width / 2}px; height: {bed_height / 2}px;"
                                    }
                                    h3 { "{bed_name}" }
                                }
                                p { "{bed_width} x {bed_height} cm | {plants_len} plantas" }
                                button {
                                    class: "delete-btn",
                                    onclick: move |_| delete_bed(&bed_id),
                                    "Eliminar"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
