use crate::router::Route;
use dioxus::prelude::*;

pub mod dashboard_v2;
pub mod tasks;

pub use dashboard_v2::{
    GardenCardV2, GardenData, HarvestItem, MaintenancePanel, MaintenanceTask, RecentHarvests,
};

#[component]
pub fn Navbar() -> Element {
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
