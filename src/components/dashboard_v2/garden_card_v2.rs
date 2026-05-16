use crate::router::Route;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct GardenData {
    pub id: String,
    pub name: String,
    pub status: String,
    pub species_count: i32,
    pub light_exposure: String,
    pub tags: Vec<String>,
    pub image_url: String,
}

#[component]
pub fn GardenCardV2(garden: GardenData) -> Element {
    let status_class = if garden.status == "Harvest Ready" {
        "garden-card-v2-badge harvest"
    } else {
        "garden-card-v2-badge"
    };

    rsx! {
        Link {
            to: Route::GardenDetail { id: garden.id.clone() },
            class: "garden-card-v2",

            div { class: "garden-card-v2-image",
                div {
                    style: "width: 100%; height: 100%; background: linear-gradient(135deg, var(--surface-container-high) 0%, var(--surface-container) 100%); display: flex; align-items: center; justify-content: center; font-size: 4rem; color: var(--outline-variant);",
                    "🌿"
                }
            }

            div { class: "garden-card-v2-content",
                div { class: "garden-card-v2-header",
                    h3 { class: "garden-card-v2-title", "{garden.name}" }
                    span { class: "{status_class}", "{garden.status}" }
                }

                p { class: "garden-card-v2-meta",
                    "{garden.species_count} especies • {garden.light_exposure}"
                }

                div { class: "garden-card-v2-tags",
                    for tag in garden.tags.iter() {
                        span { class: "garden-card-v2-tag", "{tag}" }
                    }
                }
            }
        }
    }
}
