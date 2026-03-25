use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TaskItemData {
    pub title: String,
    pub description: String,
    pub image: String,
    pub frequency: String,
    pub frequency_icon: String,
    pub badge: String,
}

#[component]
pub fn TaskListItem(task: TaskItemData, on_toggle: EventHandler<bool>) -> Element {
    rsx! {
        div { class: "task-list-item",
            div { class: "task-list-image",
                img {
                    src: "{task.image}",
                    alt: "{task.title}",
                }
            }

            div { class: "task-list-content",
                h4 { class: "task-list-title", "{task.title}" }
                p { class: "task-list-description", "{task.description}" }
            }

            div { class: "task-list-frequency",
                span { class: "task-list-frequency-text", "{task.frequency}" }
                span { class: "material-symbols-outlined task-list-frequency-icon", "{task.frequency_icon}" }
            }

            div { class: "task-list-actions",
                span { class: "task-list-badge", "{task.badge}" }
                input {
                    r#type: "checkbox",
                    class: "task-list-checkbox",
                    onclick: move |_| on_toggle.call(true),
                }
            }
        }
    }
}
