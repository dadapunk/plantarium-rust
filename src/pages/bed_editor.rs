use crate::app_state::{add_plant_to_bed, harvest_plant, remove_plant_from_bed, BEDS, PLANTS};
use crate::components::Navbar;
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn BedEditor(id: String) -> Element {
    let bed = BEDS.read().iter().find(|b| b.base.id == id).cloned();
    let plants = PLANTS.read();

    let selected_plant_id = use_signal(|| None::<String>);
    let mut action_date = use_signal(|| chrono::Local::now().format("%Y-%m-%d").to_string());

    let plants_vec: Vec<_> = plants.iter().cloned().collect();
    let bed_clone = bed.clone();

    rsx! {
        div { class: "app-container",
            Navbar {}
            div { class: "main-content",
                div { class: "header",
                    if let Some(ref b) = bed {
                        Link { to: Route::GardenDetail { id: b.garden_id.clone() }, "← Volver" }
                        h1 { "{b.name}" }
                    }
                }

                div { class: "date-selector",
                    label { "Fecha de la acción:" }
                    input {
                        r#type: "date",
                        value: "{action_date}",
                        oninput: move |evt| action_date.set(evt.value()),
                    }
                }

                div { class: "editor-container",
                    div { class: "plant-library",
                        h3 { "Biblioteca de Plantas" }
                        div { class: "plants-grid",
                            for plant in &plants_vec {
                                PlantButton {
                                    plant: plant.clone(),
                                    selected_id: selected_plant_id.clone(),
                                }
                            }
                        }
                        p { class: "hint", "Selecciona una planta y haz clic en el bancal" }
                    }

                    div { class: "canvas-container",
                        if let Some(b) = bed_clone {
                            BedCanvas {
                                bed: b,
                                plants: plants_vec.clone(),
                                selected_plant_id: selected_plant_id.clone(),
                                action_date: action_date.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PlantButton(plant: crate::app_state::Plant, selected_id: Signal<Option<String>>) -> Element {
    let plant_id = plant.base.id.clone();

    rsx! {
        button {
            class: if selected_id() == Some(plant_id.clone()) { "plant-btn selected" } else { "plant-btn" },
            onclick: move |_| {
                let current = selected_id();
                if current == Some(plant_id.clone()) {
                    selected_id.set(None);
                } else {
                    selected_id.set(Some(plant_id.clone()));
                }
            },
            span { class: "icon", "{plant.icon}" }
            span { class: "name", "{plant.name}" }
        }
    }
}

#[component]
fn BedCanvas(
    bed: crate::app_state::Bed,
    plants: Vec<crate::app_state::Plant>,
    selected_plant_id: Signal<Option<String>>,
    action_date: Signal<String>,
) -> Element {
    let bid = bed.base.id.clone();
    let bid_for_add = bed.base.id.clone();
    let mut canvas_pos = use_signal(|| (0.0f64, 0.0f64));

    rsx! {
        div {
            class: "canvas",
            style: "width: {bed.width}px; height: {bed.height}px; position: relative;",
            onmounted: move |evt| {
                spawn(async move {
                    if let Ok(rect) = evt.get_client_rect().await {
                        canvas_pos.set((rect.origin.x, rect.origin.y));
                    }
                });
            },
            onclick: move |evt| {
                if let Some(plant_id) = selected_plant_id() {
                    let (cx, cy) = canvas_pos();
                    let coords = evt.client_coordinates();
                    let x = coords.x - cx;
                    let y = coords.y - cy;
                    add_plant_to_bed(&bid_for_add, &plant_id, x, y, &action_date());
                }
            },
            for placed in bed.plants.iter() {
                if let Some(plant_info) = plants.iter().find(|p| p.base.id == placed.plant_id) {
                    PlacedPlantItem {
                        placed: placed.clone(),
                        plant_info: plant_info.clone(),
                        bed_id: bid.clone(),
                        action_date: action_date.clone(),
                    }
                }
            }
        }
    }
}

#[component]
fn PlacedPlantItem(
    placed: crate::app_state::PlacedPlant,
    plant_info: crate::app_state::Plant,
    bed_id: String,
    action_date: Signal<String>,
) -> Element {
    let placed_id = placed.base.id.clone();
    let placed_id_remove = placed.base.id.clone();
    let bid_harvest = bed_id.clone();
    let bid_remove = bed_id.clone();

    rsx! {
        div {
            class: if placed.harvested_at.is_some() { "placed-plant harvested" } else { "placed-plant" },
            style: "left: {placed.x}px; top: {placed.y}px; background: {plant_info.color};",
            span { class: "plant-icon", "{plant_info.icon}" }
            div { class: "plant-actions",
                if placed.harvested_at.is_none() {
                    button {
                        class: "harvest-btn",
                        onclick: move |_| harvest_plant(&bid_harvest, &placed_id, &action_date()),
                        "🧺"
                    }
                }
                button {
                    class: "remove-btn",
                    onclick: move |_| remove_plant_from_bed(&bid_remove, &placed_id_remove, &action_date()),
                    "✕"
                }
            }
        }
    }
}
