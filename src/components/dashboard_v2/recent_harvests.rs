use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HarvestItem {
    pub name: String,
    pub icon: String,
    pub harvested_date: String,
    pub yield_amount: String,
}

#[component]
pub fn RecentHarvests(harvests: Vec<HarvestItem>) -> Element {
    rsx! {
        div { class: "recent-harvests",
            // Content
            div { class: "recent-harvests-content",
                h2 { "Recent Harvests" }
                p { class: "recent-harvests-subtitle",
                    "Your labor is bearing fruit. View your latest yields."
                }

                div { class: "recent-harvests-list",
                    for harvest in harvests.iter() {
                        div { class: "recent-harvest-item",
                            div { class: "recent-harvest-icon", "{harvest.icon}" }
                            div { class: "recent-harvest-info",
                                h4 { "{harvest.name}" }
                                p { "Harvested {harvest.harvested_date} • {harvest.yield_amount}" }
                            }
                        }
                    }
                }

                button { class: "btn-primary-v2", "Log New Harvest" }
            }

            // Image
            div { class: "recent-harvests-image",
                div {
                    style: "width: 100%; height: 100%; background: linear-gradient(135deg, var(--tertiary-container) 0%, var(--primary-container) 100%); display: flex; align-items: center; justify-content: center; font-size: 5rem;",
                    "🥬"
                }
            }
        }
    }
}
