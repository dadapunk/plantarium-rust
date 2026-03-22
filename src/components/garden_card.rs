use crate::app_state::{DemoGarden, GardenStatus};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GardenCardProps {
    pub garden: DemoGarden,
    pub on_view: EventHandler<()>,
    pub on_edit: EventHandler<()>,
}

#[component]
pub fn GardenCard(props: GardenCardProps) -> Element {
    let status_class = match props.garden.status {
        GardenStatus::Active => "active",
        GardenStatus::NeedsWater => "needs-water",
        GardenStatus::Harvestable => "harvestable",
    };

    let status_text = match props.garden.status {
        GardenStatus::Active => "Active",
        GardenStatus::NeedsWater => "Needs Water",
        GardenStatus::Harvestable => "Harvestable",
    };

    rsx! {
        div { class: "garden-card",
            span { class: "status-badge {status_class}", "{status_text}" }

            h3 { "{props.garden.name}" }

            div { class: "garden-stats",
                div { class: "garden-stat",
                    span { "layers: {props.garden.bed_count} Raised Beds" }
                }
                div { class: "garden-stat",
                    span { "eco: {props.garden.plant_count} Healthy Plants" }
                }
                div { class: "garden-stat",
                    span { "history: Last Activity: {props.garden.last_activity}" }
                }
            }

            div { class: "card-actions",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| props.on_view.call(()),
                    "View"
                }
                button {
                    class: "btn btn-secondary",
                    onclick: move |_| props.on_edit.call(()),
                    "Edit"
                }
            }
        }
    }
}
