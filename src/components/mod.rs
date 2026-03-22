use crate::router::Route;
use dioxus::prelude::*;

mod add_garden_card;
mod garden_card;
mod header;
mod pro_tip;
mod quick_reminders;
mod stat_card;

pub use add_garden_card::AddGardenCard;
pub use garden_card::GardenCard;
pub use header::Header;
pub use pro_tip::ProTip;
pub use quick_reminders::QuickReminders;
pub use stat_card::StatCard;

#[component]
pub fn Navbar() -> Element {
    let lang = use_signal(|| "ES".to_string());

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
