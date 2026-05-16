use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::Link;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "header",
            div { class: "header-logo",
                h1 { "🌱 Plantarium" }
                span { class: "season-badge", "Spring Season: March - May" }
            }

            nav { class: "nav",
                Link {
                    to: Route::Dashboard {},
                    class: "nav-item active",
                    "📊 Dashboard"
                }
                Link {
                    to: Route::Calendar {},
                    class: "nav-item",
                    "📅 Calendar"
                }
                Link {
                    to: Route::Tasks {},
                    class: "nav-item",
                    "✅ Tasks"
                }
                Link {
                    to: Route::Journal {},
                    class: "nav-item",
                    "📝 Journal"
                }
                Link {
                    to: Route::Dashboard {}, // Settings route doesn't exist yet
                    class: "nav-item",
                    "⚙️ Settings"
                }
            }

            div { class: "header-actions",
                span { class: "header-icon", "☀️" }
                span { class: "header-icon", "🔔" }
                span { class: "header-icon", "👤" }
            }
        }
    }
}
