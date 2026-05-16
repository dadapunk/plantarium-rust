use crate::router::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GardenCardProps {
    pub id: String,
    pub name: String,
    pub bed_count: usize,
    pub plant_count: usize,
}

#[component]
pub fn GardenCard(props: GardenCardProps) -> Element {
    rsx! {
        div { class: "garden-card",
            h3 { "{props.name}" }
            div { class: "garden-stats",
                div { class: "garden-stat",
                    span { "{props.bed_count} bancales" }
                }
                div { class: "garden-stat",
                    span { "{props.plant_count} plantas" }
                }
            }
            div { class: "card-actions",
                Link {
                    to: Route::GardenDetail { id: props.id.clone() },
                    class: "btn btn-primary",
                    "Ver"
                }
            }
        }
    }
}
