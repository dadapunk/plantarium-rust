use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MaintenanceTask {
    pub title: String,
    pub location: String,
    pub due_date: String,
    pub is_critical: bool,
    pub task_type: String,
    pub icon: String,
}

#[component]
pub fn MaintenancePanel(tasks: Vec<MaintenanceTask>) -> Element {
    let task_count = tasks.len();

    rsx! {
        div { class: "maintenance-panel",
            div { class: "maintenance-panel-header",
                h2 { "Maintenance" }
                div { class: "maintenance-panel-count", "{task_count} Tasks" }
            }

            div { class: "maintenance-tasks",
                for task in tasks.iter() {
                    MaintenanceTaskItem { task: task.clone() }
                }
            }
        }
    }
}

#[component]
fn MaintenanceTaskItem(task: MaintenanceTask) -> Element {
    let badge_class = if task.is_critical {
        "maintenance-task-badge critical"
    } else {
        "maintenance-task-badge normal"
    };

    let due_text = if task.is_critical {
        rsx! { span { class: "critical", "{task.due_date}" } }
    } else {
        rsx! { "{task.due_date}" }
    };

    rsx! {
        div { class: "maintenance-task",
            div { class: "maintenance-task-checkbox" }
            div { class: "maintenance-task-content",
                h4 { "{task.title}" }
                p { class: "maintenance-task-meta",
                    "{task.location} • "
                    {due_text}
                }
                span { class: "{badge_class}",
                    "{task.task_type}"
                }
            }
        }
    }
}
