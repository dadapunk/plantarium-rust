use crate::router::Route;
use dioxus::prelude::*;

mod add_garden_card;
mod garden_card;
mod header;
mod pro_tip;
mod quick_reminders;
mod stat_card;

pub mod dashboard_v2;
pub mod tasks;

pub use dashboard_v2::{
    DashboardHeader, GardenCardV2, GardenData, HarvestItem, MaintenancePanel, MaintenanceTask,
    RecentHarvests,
};

#[component]
pub fn Navbar() -> Element {
    let _lang = use_signal(|| "ES".to_string());

    rsx! {
        nav { class: "navbar",
            Link { to: Route::Dashboard {}, "🌱 Plantarium" }
            div { class: "nav-links",
                Link { to: Route::Dashboard {}, "Jardines" }
                Link { to: Route::Calendar {}, "Calendario" }
                Link { to: Route::Journal {}, "Diario" }
                Link { to: Route::Tasks {}, "Tareas" }
            }
        }
    }
}
