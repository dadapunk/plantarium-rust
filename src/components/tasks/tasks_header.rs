use dioxus::prelude::*;

#[component]
pub fn TasksHeader(on_add_click: EventHandler<()>) -> Element {
    rsx! {
        section { class: "tasks-header",
            div { class: "tasks-header-content",
                span { class: "tasks-label", "Spring Maintenance" }
                h1 { class: "tasks-title", "Garden Tasks" }
                p { class: "tasks-subtitle",
                    "Your conservatory currently has 12 active tasks. The soil moisture levels are optimal for fertilizing the Monsteras today."
                }
            }

            button {
                class: "add-task-btn",
                onclick: move |_| on_add_click.call(()),
                span { class: "material-symbols-outlined", "add_task" }
                "Add New Task"
            }
        }
    }
}
