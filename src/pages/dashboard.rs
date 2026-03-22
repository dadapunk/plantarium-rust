use crate::app_state::{get_demo_gardens, get_demo_stats, get_demo_tasks};
use crate::components::{AddGardenCard, GardenCard, Header, ProTip, QuickReminders, StatCard};
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    let demo_gardens = use_signal(get_demo_gardens);
    let demo_stats = use_signal(get_demo_stats);
    let demo_tasks = use_signal(get_demo_tasks);

    // Clone data to avoid lifetime issues in RSX
    let gardens_clone = demo_gardens.read().clone();
    let stats_clone = demo_stats.read().clone();
    let tasks_clone = demo_tasks.read().clone();

    rsx! {
        div { class: "dashboard-page",
            Header {}

            // Stats Section
            section { class: "stats-section",
                StatCard {
                    title: "Total Gardens".to_string(),
                    value: stats_clone.total_gardens.to_string(),
                    icon: "🌱".to_string()
                }
                StatCard {
                    title: "Total Plants".to_string(),
                    value: stats_clone.total_plants.to_string(),
                    icon: "🌿".to_string()
                }
                StatCard {
                    title: "Upcoming Tasks".to_string(),
                    value: stats_clone.upcoming_tasks.to_string(),
                    icon: "✅".to_string()
                }
                StatCard {
                    title: "Recent Harvests".to_string(),
                    value: stats_clone.recent_harvests.to_string(),
                    icon: "🌾".to_string()
                }
            }

            // Gardens Section
            section { class: "gardens-section",
                div { class: "section-header",
                    h2 { "Your Gardens" }
                    div { class: "view-toggle",
                        button { class: "toggle-btn active", "📊 Grid" }
                        button { class: "toggle-btn", "📋 List" }
                    }
                }

                div { class: "gardens-grid",
                    for garden in gardens_clone.iter() {
                        GardenCard {
                            key: "{garden.id:?}",
                            garden: garden.clone(),
                            on_view: move |_| {
                                // TODO: Navigate to garden detail
                                println!("View garden");
                            },
                            on_edit: move |_| {
                                // TODO: Navigate to garden edit
                                println!("Edit garden");
                            }
                        }
                    }

                    AddGardenCard {
                        on_click: move |_| {
                            // TODO: Show add garden modal
                            println!("Add new garden clicked");
                        }
                    }
                }
            }

            // Pro Tip
            ProTip {}

            // Quick Reminders
            QuickReminders {
                tasks: tasks_clone
            }
        }
    }
}
