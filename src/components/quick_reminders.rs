use crate::app_state::Task;
use crate::router::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct QuickRemindersProps {
    pub tasks: Vec<Task>,
}

#[component]
pub fn QuickReminders(props: QuickRemindersProps) -> Element {
    let pending: Vec<_> = props.tasks.iter().filter(|t| !t.completed).collect();

    rsx! {
        div { class: "quick-reminders",
            h3 { "⏰ Tareas Pendientes" }
            p { "Tienes {pending.len()} tareas sin completar." }

            div { class: "reminder-list",
                for task in pending.iter().take(5) {
                    div { class: "reminder-item",
                        div { class: "reminder-info",
                            span { "{task.title}" }
                        }
                        span { class: "reminder-time", "{task.date}" }
                    }
                }
            }

            Link { to: Route::Tasks {}, class: "go-to-tasks", "Ir a Tareas →" }
        }
    }
}
