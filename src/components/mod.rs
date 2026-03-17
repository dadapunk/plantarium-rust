use crate::router::Route;
use dioxus::prelude::*;

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
