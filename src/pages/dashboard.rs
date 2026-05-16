use crate::app_state::{create_garden, BEDS, GARDENS, PLOT_ACTIONS, TASKS};
use crate::components::{AddGardenCard, GardenCard, Navbar, ProTip, QuickReminders, StatCard};
use dioxus::prelude::*;

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
    let harvests = actions
        .iter()
        .filter(|a| a.action == crate::app_state::PlotActionType::Harvested)
        .count();

    let total_plants: usize = active_beds
        .iter()
        .map(|b| b.plants.iter().filter(|p| p.harvested_at.is_none()).count())
        .sum();
    let total_gardens = active_gardens.len();
    let upcoming = active_tasks.iter().filter(|t| !t.completed).count();

    let garden_cards: Vec<_> = active_gardens
        .iter()
        .map(|garden| {
            let gid = garden.base.id.clone();
            let gname = garden.name.clone();
            let gbeds: Vec<_> = active_beds.iter().filter(|b| b.garden_id == gid).collect();
            let bed_count = gbeds.len();
            let plant_count: usize = gbeds
                .iter()
                .map(|b| b.plants.iter().filter(|p| p.harvested_at.is_none()).count())
                .sum();
            rsx! {
                GardenCard {
                    key: "{gid}",
                    id: gid,
                    name: gname,
                    bed_count,
                    plant_count,
                }
            }
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
            div { class: "main-content",
                div { class: "header",
                    h1 { "Mis Jardines" }
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

                section { class: "stats-section",
                    StatCard { title: "Jardines".to_string(), value: total_gardens.to_string(), icon: "🌱".to_string() }
                    StatCard { title: "Plantas".to_string(), value: total_plants.to_string(), icon: "🌿".to_string() }
                    StatCard { title: "Tareas Pendientes".to_string(), value: upcoming.to_string(), icon: "✅".to_string() }
                    StatCard { title: "Cosechas".to_string(), value: harvests.to_string(), icon: "🌾".to_string() }
                }

                div { class: "gardens-section",
                    div { class: "gardens-grid",
                        {garden_cards.into_iter()}
                        AddGardenCard {
                            on_click: move |_| show_add.toggle()
                        }
                    }
                }

                ProTip {}
                QuickReminders { tasks: active_tasks.clone() }
            }
        }
    }
}
