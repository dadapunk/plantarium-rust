use crate::app_state::DemoTask;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct QuickRemindersProps {
    pub tasks: Vec<DemoTask>,
}

#[component]
pub fn QuickReminders(props: QuickRemindersProps) -> Element {
    rsx! {
        div { class: "quick-reminders",
            h3 { "⏰ Quick Reminders" }
            p { "You have {props.tasks.len()} tasks due today." }

            div { class: "reminder-list",
                for task in props.tasks.iter() {
                    div { class: "reminder-item",
                        div { class: "reminder-info",
                            span { "{task.title}" }
                        }
                        span {
                            class: if task.completed { "reminder-status" } else { "reminder-time" },
                            "{task.time}"
                        }
                    }
                }
            }

            a { href: "#", class: "go-to-tasks", "Go to Tasks →" }
        }
    }
}
