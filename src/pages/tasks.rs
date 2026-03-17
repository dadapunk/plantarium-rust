use crate::app_state::{create_task, delete_task, toggle_task, TaskType, TASKS};
use crate::components::Navbar;
use dioxus::prelude::*;

#[component]
pub fn Tasks() -> Element {
    let mut filter = use_signal(|| "all".to_string());
    let mut show_add = use_signal(|| false);
    let mut new_title = use_signal(|| String::new());
    let mut new_date = use_signal(|| String::new());
    let mut new_type = use_signal(|| TaskType::Custom);

    let tasks = TASKS.read();
    let filtered: Vec<_> = tasks
        .iter()
        .filter(|t| t.base.deleted_at.is_none())
        .filter(|t| match filter().as_str() {
            "pending" => !t.completed,
            "completed" => t.completed,
            _ => true,
        })
        .collect();

    let mut add_task = move || {
        if !new_title().trim().is_empty() && !new_date().is_empty() {
            create_task(&new_title(), &new_date(), new_type());
            new_title.set(String::new());
            new_date.set(String::new());
            show_add.set(false);
        }
    };

    rsx! {
        div { class: "app-container",
            Navbar {}
            div { class: "main-content",
                div { class: "header",
                    h1 { "Tareas" }
                    button {
                        class: "add-btn",
                        onclick: move |_| show_add.toggle(),
                        if show_add() { "Cancelar" } else { "+ Añadir Tarea" }
                    }
                }

                if show_add() {
                    div { class: "add-form",
                        input {
                            r#type: "text",
                            placeholder: "Título de la tarea",
                            value: "{new_title}",
                            oninput: move |evt| new_title.set(evt.value()),
                        }
                        select {
                            onchange: move |evt| {
                                new_type.set(match evt.value().as_str() {
                                    "sowing" => TaskType::Sowing,
                                    "watering" => TaskType::Watering,
                                    "harvest" => TaskType::Harvest,
                                    "fertilizing" => TaskType::Fertilizing,
                                    _ => TaskType::Custom,
                                });
                            },
                            option { value: "sowing", "Siembra" }
                            option { value: "watering", "Riego" }
                            option { value: "harvest", "Cosecha" }
                            option { value: "fertilizing", "Fertilizar" }
                            option { value: "custom", "Personalizado" }
                        }
                        input {
                            r#type: "date",
                            value: "{new_date}",
                            oninput: move |evt| new_date.set(evt.value()),
                        }
                        button { onclick: move |_| add_task(), "Guardar" }
                    }
                }

                div { class: "filters",
                    select {
                        value: "{filter}",
                        onchange: move |evt| filter.set(evt.value()),
                        option { value: "all", "Todos" }
                        option { value: "pending", "Pendientes" }
                        option { value: "completed", "Completadas" }
                    }
                }

                div { class: "tasks-list",
                    for task in filtered {
                        TaskItem { task: task.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn TaskItem(task: crate::app_state::Task) -> Element {
    let _editing = use_signal(|| false);

    let get_type_label = |t: &TaskType| -> &'static str {
        match t {
            TaskType::Sowing => "Siembra",
            TaskType::Watering => "Riego",
            TaskType::Harvest => "Cosecha",
            TaskType::Fertilizing => "Fertilizar",
            TaskType::Custom => "Personalizado",
        }
    };

    let task_id = task.base.id.clone();
    let task_id_delete = task.base.id.clone();

    rsx! {
        div { class: if task.completed { "task-card completed" } else { "task-card" },
            button {
                class: if task.completed { "checkbox checked" } else { "checkbox" },
                onclick: move |_| toggle_task(&task_id),
            }
            div { class: "task-content",
                span { class: "task-title", "{task.title}" }
                span { class: "task-meta",
                    span { class: "task-type", "{get_type_label(&task.r#type)}" }
                    span { class: "task-date", "{task.date}" }
                }
            }
            button {
                class: "delete-btn",
                onclick: move |_| delete_task(&task_id_delete),
                "×"
            }
        }
    }
}
