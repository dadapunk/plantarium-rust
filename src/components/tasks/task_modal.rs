use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NewTask {
    pub title: String,
    pub date: String,
    pub task_type: String,
}

#[component]
pub fn TaskModal(
    is_open: bool,
    on_close: EventHandler<()>,
    on_submit: EventHandler<NewTask>,
) -> Element {
    let mut title = use_signal(String::new);
    let mut date = use_signal(String::new);
    let mut task_type = use_signal(|| "routine".to_string());

    if !is_open {
        return rsx! {};
    }

    rsx! {
        div { class: "task-modal-overlay",
            onclick: move |_| on_close.call(()),

            div { class: "task-modal",
                onclick: move |evt| evt.stop_propagation(),

                h2 { class: "task-modal-title", "Add New Task" }

                div { class: "task-form",
                    div {
                        label { class: "task-form-label", "Title" }
                        input {
                            r#type: "text",
                            class: "task-form-input",
                            placeholder: "Task title",
                            value: "{title}",
                            oninput: move |evt| title.set(evt.value()),
                        }
                    }

                    div {
                        label { class: "task-form-label", "Due Date" }
                        input {
                            r#type: "date",
                            class: "task-form-input",
                            value: "{date}",
                            oninput: move |evt| date.set(evt.value()),
                        }
                    }

                    div {
                        label { class: "task-form-label", "Type" }
                        select {
                            class: "task-form-select",
                            value: "{task_type}",
                            onchange: move |evt| task_type.set(evt.value()),
                            option { value: "urgent", "Urgent Care" }
                            option { value: "routine", "Routine Maintenance" }
                            option { value: "seasonal", "Seasonal Planning" }
                        }
                    }

                    div { class: "task-form-buttons",
                        button {
                            class: "task-form-cancel",
                            onclick: move |_| on_close.call(()),
                            "Cancel"
                        }
                        button {
                            class: "task-form-submit",
                            onclick: move |_| {
                                if !title().trim().is_empty() && !date().is_empty() {
                                    on_submit.call(NewTask {
                                        title: title(),
                                        date: date(),
                                        task_type: task_type(),
                                    });
                                    title.set(String::new());
                                    date.set(String::new());
                                }
                            },
                            "Add Task"
                        }
                    }
                }
            }
        }
    }
}
