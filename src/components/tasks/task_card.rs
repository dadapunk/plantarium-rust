use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TaskCardData {
    pub icon: String,
    pub badge: String,
    pub title: String,
    pub description: String,
    pub plant_image: String,
    pub location: String,
    pub date: String,
}

#[component]
pub fn TaskCard(task: TaskCardData, on_toggle: EventHandler<()>) -> Element {
    rsx! {
        div { class: "task-card",
            div { class: "task-card-content",
                div { class: "task-card-header",
                    span { class: "material-symbols-outlined task-card-icon", "{task.icon}" }
                    span { class: "task-card-badge", "{task.badge}" }
                }

                h3 { class: "task-card-title", "{task.title}" }
                p { class: "task-card-description", "{task.description}" }

                div { class: "task-card-footer",
                    div { class: "task-card-plants",
                        img {
                            class: "task-card-plant-thumb",
                            src: "{task.plant_image}",
                            alt: "Plant thumbnail"
                        }
                    }
                    span { class: "task-card-location", "{task.location}" }
                }
            }

            div { class: "task-card-actions",
                input {
                    r#type: "checkbox",
                    class: "task-card-checkbox",
                    onclick: move |_| on_toggle.call(())
                }
                span { class: "task-card-date", "{task.date}" }
            }
        }
    }
}
