use dioxus::prelude::*;

#[component]
pub fn FertilizerAlertCard(on_action: EventHandler<()>) -> Element {
    rsx! {
        div { class: "alert-card",
            div {
                h3 { class: "alert-card-title", "Fertilizer Alert" }
                p { class: "alert-card-text", "The Orchid collection is due for their bi-monthly nutrient feeding." }
                button {
                    class: "alert-card-btn",
                    onclick: move |_| on_action.call(()),
                    "Mark as Done"
                }
            }

            span {
                class: "material-symbols-outlined alert-card-icon",
                "eco"
            }
        }
    }
}
