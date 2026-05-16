use crate::app_state::{create_task, delete_task, toggle_task, Task, TaskType, TASKS};
use crate::components::tasks::{
    CalendarWidget, FeaturedCard, FertilizerAlertCard, NewTask, TaskCard, TaskCardData,
    TaskItemData, TaskListItem, TaskModal, TasksHeader,
};
use crate::components::Navbar;
use dioxus::prelude::*;

fn task_to_card_data(task: &Task) -> TaskCardData {
    let (icon, badge) = match task.r#type {
        TaskType::Watering => ("water_drop".to_string(), "Hydration".to_string()),
        TaskType::Fertilizing => ("compost".to_string(), "Fertilizer".to_string()),
        TaskType::Harvest => ("agriculture".to_string(), "Harvest".to_string()),
        TaskType::Sowing => ("grass".to_string(), "Sowing".to_string()),
        TaskType::Custom => ("task".to_string(), "Custom".to_string()),
    };
    TaskCardData {
        icon,
        badge,
        title: task.title.clone(),
        description: format!("Scheduled for {}", task.date),
        plant_image: String::new(),
        location: String::new(),
        date: task.date.clone(),
    }
}

fn task_to_item_data(task: &Task) -> TaskItemData {
    let (frequency_icon, badge) = match task.r#type {
        TaskType::Watering => ("water_drop".to_string(), "Watering".to_string()),
        TaskType::Fertilizing => ("compost".to_string(), "Fertilizing".to_string()),
        TaskType::Harvest => ("agriculture".to_string(), "Harvest".to_string()),
        TaskType::Sowing => ("grass".to_string(), "Sowing".to_string()),
        TaskType::Custom => ("task".to_string(), "Custom".to_string()),
    };
    TaskItemData {
        title: task.title.clone(),
        description: format!("Scheduled for {}", task.date),
        image: String::new(),
        frequency: task.date.clone(),
        frequency_icon,
        badge,
    }
}

#[component]
pub fn Tasks() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut filter_type = use_signal(|| Option::<TaskType>::None);
    let mut filter_status = use_signal(|| Option::<bool>::None);

    let all_tasks = TASKS.read();
    let filtered_tasks: Vec<Task> = all_tasks
        .iter()
        .filter(|t| t.base.deleted_at.is_none())
        .filter(|t| {
            if let Some(ref ft) = *filter_type.read() {
                if t.r#type != *ft {
                    return false;
                }
            }
            if let Some(fs) = *filter_status.read() {
                if t.completed != fs {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    let pending_tasks: Vec<Task> = filtered_tasks
        .iter()
        .filter(|t| !t.completed)
        .cloned()
        .collect();
    let completed_tasks: Vec<Task> = filtered_tasks
        .iter()
        .filter(|t| t.completed)
        .cloned()
        .collect();

    let pending_task_ids: Vec<String> = pending_tasks.iter().map(|t| t.base.id.clone()).collect();
    let completed_task_ids: Vec<String> =
        completed_tasks.iter().map(|t| t.base.id.clone()).collect();

    let handle_add_task = move |new_task: NewTask| {
        let task_type = match new_task.task_type.as_str() {
            "urgent" => TaskType::Watering,
            "routine" => TaskType::Custom,
            "seasonal" => TaskType::Sowing,
            _ => TaskType::Custom,
        };
        create_task(&new_task.title, &new_task.date, task_type);
        show_modal.set(false);
    };

    let handle_toggle = move |id: String| {
        toggle_task(&id);
    };

    let handle_delete = move |id: String| {
        delete_task(&id);
    };

    let active_type = filter_type.read().clone();
    let active_status = filter_status.read().clone();

    // Pre-build task cards to avoid closure issues
    let pending_card_elements: Vec<_> = pending_tasks
        .iter()
        .zip(pending_task_ids.iter())
        .map(|(task, id)| {
            let task_data = task_to_card_data(task);
            let task_id = id.clone();
            (task_data, task_id)
        })
        .collect();

    let completed_item_elements: Vec<_> = completed_tasks
        .iter()
        .zip(completed_task_ids.iter())
        .map(|(task, id)| {
            let task_data = task_to_item_data(task);
            let task_id = id.clone();
            (task_data, task_id)
        })
        .collect();

    // Separate cards from handlers
    let pending_cards: Vec<TaskCardData> = pending_card_elements
        .iter()
        .map(|(d, _)| d.clone())
        .collect();
    let pending_ids_clone: Vec<String> = pending_card_elements
        .iter()
        .map(|(_, id)| id.clone())
        .collect();

    let completed_items: Vec<TaskItemData> = completed_item_elements
        .iter()
        .map(|(d, _)| d.clone())
        .collect();
    let completed_ids_clone: Vec<String> = completed_item_elements
        .iter()
        .map(|(_, id)| id.clone())
        .collect();

    rsx! {
        div { class: "app-container",
            Navbar {}

            div { class: "main-content",
                main { class: "tasks-main",
                TasksHeader {
                    on_add_click: move |_| show_modal.set(true),
                }

                // Filter UI
                div { class: "task-filters",
                    div { class: "filter-group",
                        span { class: "filter-label", "Type:" }
                        button {
                            class: "filter-btn",
                            onclick: move |_| filter_type.set(None),
                            if active_type.is_none() { "All (active)" } else { "All" }
                        }
                        button {
                            class: "filter-btn",
                            onclick: move |_| filter_type.set(Some(TaskType::Watering)),
                            if active_type == Some(TaskType::Watering) { "Watering (active)" } else { "Watering" }
                        }
                        button {
                            class: "filter-btn",
                            onclick: move |_| filter_type.set(Some(TaskType::Fertilizing)),
                            if active_type == Some(TaskType::Fertilizing) { "Fertilizing (active)" } else { "Fertilizing" }
                        }
                        button {
                            class: "filter-btn",
                            onclick: move |_| filter_type.set(Some(TaskType::Harvest)),
                            if active_type == Some(TaskType::Harvest) { "Harvest (active)" } else { "Harvest" }
                        }
                        button {
                            class: "filter-btn",
                            onclick: move |_| filter_type.set(Some(TaskType::Sowing)),
                            if active_type == Some(TaskType::Sowing) { "Sowing (active)" } else { "Sowing" }
                        }
                        button {
                            class: "filter-btn",
                            onclick: move |_| filter_type.set(Some(TaskType::Custom)),
                            if active_type == Some(TaskType::Custom) { "Custom (active)" } else { "Custom" }
                        }
                    }
                    div { class: "filter-group",
                        span { class: "filter-label", "Status:" }
                        button {
                            class: "filter-btn",
                            onclick: move |_| filter_status.set(None),
                            if active_status.is_none() { "All (active)" } else { "All" }
                        }
                        button {
                            class: "filter-btn",
                            onclick: move |_| filter_status.set(Some(false)),
                            if active_status == Some(false) { "Pending (active)" } else { "Pending" }
                        }
                        button {
                            class: "filter-btn",
                            onclick: move |_| filter_status.set(Some(true)),
                            if active_status == Some(true) { "Done (active)" } else { "Done" }
                        }
                    }
                }

                div { class: "tasks-grid",
                    // Sidebar (Calendar + Alerts)
                    aside { class: "tasks-sidebar",
                        CalendarWidget {}
                        FertilizerAlertCard {
                            on_action: move |_| {},
                        }
                    }

                    // Main Tasks Content
                    div { class: "tasks-content",
                        // Pending Tasks Section
                        section { class: "task-category",
                            div { class: "task-category-header",
                                h2 { class: "task-category-title", "Pending Tasks" }
                                span { class: "task-category-count urgent", "{pending_tasks.len()} Tasks" }
                            }

                            if pending_tasks.is_empty() {
                                div { class: "empty-state", "No pending tasks" }
                            } else {
                                div { class: "task-cards-grid",
                                    for (idx, card) in pending_cards.iter().enumerate() {
                                        TaskCard {
                                            key: "{idx}",
                                            task: card.clone(),
                                            on_toggle: |_| {},
                                        }
                                    }
                                }
                            }
                        }

                        // Completed Tasks Section
                            if !completed_tasks.is_empty() {
                            section { class: "task-category",
                                div { class: "task-category-header",
                                    h2 { class: "task-category-title", "Completed" }
                                    span { class: "task-category-count routine", "{completed_tasks.len()} Tasks" }
                                }

                                div { class: "task-list",
                                    for (idx, item) in completed_items.iter().enumerate() {
                                        TaskListItem {
                                            key: "{idx}",
                                            task: item.clone(),
                                            on_toggle: |_| {},
                                        }
                                    }
                                }
                            }
                        }

                        // Featured Card (Journal Entry)
                        FeaturedCard {
                            on_action: move |_| {},
                        }
                    }
                }
            }
            }

            // Floating Action Button
            button {
                class: "fab",
                onclick: move |_| show_modal.set(true),
                span { class: "material-symbols-outlined", "edit" }
            }

            // Task Modal
            TaskModal {
                is_open: *show_modal.read(),
                on_close: move |_| show_modal.set(false),
                on_submit: handle_add_task,
            }
        }
    }
}
