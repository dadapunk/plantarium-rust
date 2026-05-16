use crate::app_state::{create_journal_entry, delete_journal_entry, update_journal_entry, JOURNAL};
use crate::components::Navbar;
use dioxus::prelude::*;

#[component]
pub fn Journal() -> Element {
    let mut show_add = use_signal(|| false);
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut new_date = use_signal(|| chrono::Local::now().format("%Y-%m-%d").to_string());
    let mut new_content = use_signal(|| String::new());

    let journal = JOURNAL.read();
    let mut sorted: Vec<_> = journal
        .iter()
        .filter(|j| j.base.deleted_at.is_none())
        .cloned()
        .collect();
    sorted.sort_by(|a, b| b.date.cmp(&a.date));

    let mut save_entry = move || {
        if !new_content().trim().is_empty() {
            if let Some(id) = editing_id() {
                update_journal_entry(&id, &new_content());
            } else {
                create_journal_entry(&new_date(), &new_content());
            }
            new_content.set(String::new());
            editing_id.set(None);
            show_add.set(false);
        }
    };

    let mut cancel_edit = move || {
        new_content.set(String::new());
        editing_id.set(None);
        show_add.set(false);
    };

    rsx! {
        div { class: "app-container",
            Navbar {}
            div { class: "main-content",
                div { class: "header",
                    h1 { "Diario de la Huerta" }
                    button {
                        class: "add-btn",
                        onclick: move |_| {
                            if show_add() {
                                cancel_edit();
                            } else {
                                show_add.set(true);
                            }
                        },
                        if show_add() { "Cancelar" } else { "+ Nueva Nota" }
                    }
                }

                if show_add() {
                    div { class: "editor",
                        input {
                            r#type: "date",
                            value: "{new_date}",
                            oninput: move |evt| new_date.set(evt.value()),
                        }
                        textarea {
                            placeholder: "Escribe tu nota en markdown...",
                            value: "{new_content}",
                            oninput: move |evt| new_content.set(evt.value()),
                            rows: 10,
                        }
                        div { class: "editor-actions",
                            button { onclick: move |_| save_entry(), "Guardar Nota" }
                        }
                    }
                }

                div { class: "entries-list",
                    for entry in sorted {
                        JournalEntry { entry: entry.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn JournalEntry(entry: crate::app_state::JournalEntry) -> Element {
    let _entry_id = entry.base.id.clone();
    let entry_id_delete = entry.base.id.clone();

    rsx! {
        div { class: "entry-card",
            div { class: "entry-header",
                span { class: "entry-date", "{entry.date}" }
                div { class: "entry-actions",
                    button {
                        onclick: move |_| {
                            // Edit functionality could be added here
                        },
                        "Editar"
                    }
                    button {
                        class: "delete",
                        onclick: move |_| delete_journal_entry(&entry_id_delete),
                        "Eliminar"
                    }
                }
            }
            div { class: "entry-content",
                pre { "{entry.content}" }
            }
        }
    }
}
