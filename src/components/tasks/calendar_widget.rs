use dioxus::prelude::*;

#[component]
pub fn CalendarWidget() -> Element {
    let current_month = use_signal(|| "April 2024".to_string());

    let days_in_week = ["M", "T", "W", "T", "F", "S", "S"];
    let calendar_days: Vec<(i32, &str, Option<&str>)> = vec![
        (25, "prev", None),
        (26, "prev", None),
        (27, "prev", None),
        (28, "prev", None),
        (29, "prev", None),
        (30, "prev", None),
        (1, "current", None),
        (2, "current", None),
        (3, "current", Some("urgent")),
        (4, "current", None),
        (5, "current", None),
        (6, "current", None),
        (7, "current", None),
        (8, "current", None),
        (9, "current", Some("routine")),
        (10, "current", None),
        (11, "current", None),
        (12, "current", None),
        (13, "current", None),
        (14, "current", None),
        (15, "current", None),
        (16, "current", None),
        (17, "current", None),
        (18, "current", None),
        (19, "current", None),
        (20, "current", None),
        (21, "current", None),
        (22, "current", None),
        (23, "current", None),
        (24, "current", None),
        (25, "current", None),
        (26, "current", None),
        (27, "current", None),
        (28, "current", None),
        (29, "current", None),
        (30, "current", None),
    ];

    rsx! {
        div { class: "calendar-widget",
            div { class: "calendar-header",
                h3 { class: "calendar-title", "{current_month}" }
                div { class: "calendar-nav",
                    button { class: "calendar-nav-btn",
                        span { class: "material-symbols-outlined", "chevron_left" }
                    }
                    button { class: "calendar-nav-btn",
                        span { class: "material-symbols-outlined", "chevron_right" }
                    }
                }
            }

            div { class: "calendar-grid",
                for day_name in days_in_week.iter() {
                    span { class: "calendar-day-name", "{day_name}" }
                }

                for (day, month_type, task_type) in calendar_days.iter() {
                    div {
                        class: if *month_type == "prev" {
                            "calendar-day other-month"
                        } else {
                            "calendar-day"
                        },
                        span { "{day}" }
                        if let Some(task_marker) = task_type {
                            span {
                                class: if *task_marker == "urgent" {
                                    "task-dot urgent"
                                } else {
                                    "task-dot routine"
                                }
                            }
                        }
                    }
                }
            }

            div { class: "priority-categories",
                h4 { class: "priority-categories-title", "Priority Categories" }

                div { class: "priority-item",
                    div { class: "priority-dot urgent" }
                    span { "Urgent Care" }
                }

                div { class: "priority-item",
                    div { class: "priority-dot routine" }
                    span { "Routine Maintenance" }
                }

                div { class: "priority-item",
                    div { class: "priority-dot seasonal" }
                    span { "Seasonal Planning" }
                }
            }
        }
    }
}
