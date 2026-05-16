use crate::app_state::{create_garden, BEDS, GARDENS, PLOT_ACTIONS, TASKS};
use crate::components::{
    GardenCardV2, GardenData, HarvestItem, MaintenancePanel, MaintenanceTask, Navbar,
    RecentHarvests,
};
use dioxus::prelude::*;
use std::collections::HashSet;

fn garden_status(
    garden_id: &str,
    beds: &[crate::app_state::Bed],
    tasks: &[crate::app_state::Task],
) -> String {
    let has_pending = tasks.iter().any(|t| !t.completed);
    let garden_beds: Vec<_> = beds.iter().filter(|b| b.garden_id == garden_id).collect();
    let harvestable = garden_beds
        .iter()
        .any(|b| b.plants.iter().any(|p| p.harvested_at.is_none()));
    if harvestable {
        "Harvest Ready".to_string()
    } else if has_pending {
        "Needs Care".to_string()
    } else {
        "Active".to_string()
    }
}

#[component]
pub fn Dashboard() -> Element {
    let mut show_add = use_signal(|| false);
    let mut new_name = use_signal(|| String::new());

    let gardens = GARDENS.read();
    let beds = BEDS.read();
    let tasks = TASKS.read();
    let actions = PLOT_ACTIONS.read();

    let active_gardens: Vec<_> = gardens
        .iter()
        .filter(|g| g.base.deleted_at.is_none())
        .cloned()
        .collect();
    let active_beds: Vec<_> = beds
        .iter()
        .filter(|b| b.base.deleted_at.is_none())
        .cloned()
        .collect();
    let active_tasks: Vec<_> = tasks
        .iter()
        .filter(|t| t.base.deleted_at.is_none())
        .cloned()
        .collect();
    let harvests: Vec<_> = actions
        .iter()
        .filter(|a| a.action == crate::app_state::PlotActionType::Harvested)
        .cloned()
        .collect();
    drop((gardens, beds, tasks, actions));

    let total_species: i32 = active_beds
        .iter()
        .flat_map(|b| b.plants.iter().map(|p| &p.plant_id))
        .collect::<HashSet<_>>()
        .len() as i32;

    let garden_cards: Vec<GardenData> = active_gardens
        .iter()
        .map(|g| {
            let garden_beds: Vec<_> = active_beds
                .iter()
                .filter(|b| b.garden_id == g.base.id)
                .collect();
            let unique_species: HashSet<_> = garden_beds
                .iter()
                .flat_map(|b| b.plants.iter().map(|p| &p.plant_id))
                .collect();
            GardenData {
                id: g.base.id.clone(),
                name: g.name.clone(),
                status: garden_status(&g.base.id, &active_beds, &active_tasks),
                species_count: unique_species.len() as i32,
                light_exposure: "Full Sun".to_string(),
                tags: vec![],
                image_url: "".to_string(),
            }
        })
        .collect();

    let harvest_items: Vec<HarvestItem> = harvests
        .iter()
        .map(|h| HarvestItem {
            name: h.plant_id.clone(),
            icon: "🌿".to_string(),
            harvested_date: h.date.clone(),
            yield_amount: String::new(),
        })
        .collect();

    let pending: Vec<_> = active_tasks.iter().filter(|t| !t.completed).collect();
    let maintenance_tasks: Vec<MaintenanceTask> = pending
        .iter()
        .map(|t| MaintenanceTask {
            title: t.title.clone(),
            location: String::new(),
            due_date: t.date.clone(),
            is_critical: false,
            task_type: "Task".to_string(),
            icon: "📋".to_string(),
        })
        .collect();

    let mut add_garden = move || {
        let name = new_name();
        if !name.trim().is_empty() {
            create_garden(&name);
            new_name.set(String::new());
            show_add.set(false);
        }
    };

    rsx! {
        div { class: "app-container",
            Navbar {}
            div { class: "dashboard-v2",
                header { class: "dashboard-hero",
                    p { class: "dashboard-hero-label",
                        span { class: "dashboard-hero-label-line" }
                        "El Invernadero Digital"
                    }
                    h1 {
                        "Tu conservatorio está "
                        em { "prosperando" }
                        "."
                    }
                    div { class: "hero-stats",
                        span { "{active_gardens.len()} jardines" }
                        span { "•" }
                        span { "{total_species} especies" }
                        span { "•" }
                        span { "{harvests.len()} cosechas" }
                    }
                    button {
                        class: "add-btn",
                        onclick: move |_| show_add.toggle(),
                        if show_add() { "Cancelar" } else { "+ Nuevo Jardín" }
                    }
                }

                if show_add() {
                    div { class: "add-form",
                        input {
                            r#type: "text",
                            placeholder: "Nombre del jardín",
                            value: "{new_name}",
                            oninput: move |evt| new_name.set(evt.value()),
                        }
                        button { onclick: move |_| add_garden(), "Crear" }
                    }
                }

                if active_gardens.is_empty() {
                    div { class: "empty-state",
                        p { "Aún no tienes jardines. ¡Crea tu primero!" }
                    }
                } else {
                    div { class: "dashboard-grid",
                        section { class: "dashboard-grid-main",
                            div { class: "gardens-section-header",
                                div {
                                    h2 { class: "gardens-section-title", "Tus Jardines" }
                                    p { class: "gardens-section-subtitle",
                                        "Cultivando {total_species} especies en {active_gardens.len()} espacios."
                                    }
                                }
                            }

                            div { class: "gardens-cards-grid",
                                for card in &garden_cards {
                                    GardenCardV2 {
                                        key: "{card.id}",
                                        garden: card.clone(),
                                    }
                                }
                            }

                            if !harvest_items.is_empty() {
                                RecentHarvests { harvests: harvest_items }
                            }
                        }

                        section { class: "dashboard-grid-sidebar",
                            if !maintenance_tasks.is_empty() {
                                MaintenancePanel { tasks: maintenance_tasks }
                            }
                        }
                    }
                }
            }
        }
    }
}
