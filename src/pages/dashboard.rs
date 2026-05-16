use crate::app_state::{get_demo_gardens_v2, get_demo_harvests, get_demo_maintenance_tasks};
use crate::components::{
    DashboardHeader, GardenCardV2, GardenData, MaintenancePanel, RecentHarvests,
};
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    let demo_gardens = use_signal(get_demo_gardens_v2);
    let demo_harvests = use_signal(get_demo_harvests);
    let demo_maintenance_tasks = use_signal(get_demo_maintenance_tasks);

    rsx! {
        div { class: "dashboard-v2",
            DashboardHeader {}

            main { class: "dashboard-main",
                // Hero Section
                header { class: "dashboard-hero",
                    p { class: "dashboard-hero-label",
                        span { class: "dashboard-hero-label-line" }
                        "The Digital Conservatory"
                    }
                    h1 {
                        "Your conservatory is "
                        em { "thriving" }
                        "."
                    }
                }

                // Bento Grid
                div { class: "dashboard-grid",
                    // Main Content
                    section { class: "dashboard-grid-main",
                        // Gardens Section Header (sibling of grid and recent-harvests)
                        div { class: "gardens-section-header",
                            div {
                                h2 { class: "gardens-section-title", "Your Gardens" }
                                p { class: "gardens-section-subtitle",
                                    "Nurturing 24 species across 4 micro-climates."
                                }
                            }
                            a { href: "#", class: "gardens-section-link",
                                "Explore All"
                                span { style: "margin-left: 8px;", "→" }
                            }
                        }

                        // Gardens Cards Grid (sibling of header and recent-harvests)
                        div { class: "gardens-cards-grid",
                            GardenGrid { gardens: demo_gardens }
                        }

                        // Recent Harvests (sibling of header and grid)
                        RecentHarvests { harvests: demo_harvests.read().clone() }
                    }

                    // Sidebar
                    section { class: "dashboard-grid-sidebar",
                        MaintenancePanel { tasks: demo_maintenance_tasks.read().clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn GardenGrid(gardens: Signal<Vec<GardenData>>) -> Element {
    let gardens_list = gardens.read().clone();

    rsx! {
        for (index, garden) in gardens_list.iter().enumerate() {
            GardenCardV2 {
                key: "{index}",
                garden: garden.clone(),
                on_click: move |_| {
                    println!("Garden clicked");
                }
            }
        }
    }
}
