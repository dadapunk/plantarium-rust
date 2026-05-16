use crate::app_state::{create_journal_entry, delete_journal_entry, update_journal_entry, JOURNAL};
use crate::components::Navbar;
use dioxus::prelude::*;

fn render_markdown(content: &str) -> String {
    use ammonia::Builder;
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Builder::default()
        .url_relative(ammonia::UrlRelative::Deny)
        .clean(&html_output)
        .to_string()
}

#[component]
pub fn Journal() -> Element {
    let mut show_add = use_signal(|| false);
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_date = use_signal(|| chrono::Local::now().format("%Y-%m-%d").to_string());
    let mut new_date = use_signal(|| chrono::Local::now().format("%Y-%m-%d").to_string());
    let mut new_content = use_signal(|| String::new());

    let journal = JOURNAL.read();
    let mut sorted: Vec<_> = journal
        .iter()
        .filter(|j| j.base.deleted_at.is_none())
        .cloned()
        .collect();
    sorted.sort_by(|a, b| b.date.cmp(&a.date));

    let start_edit = move |(id, date, content): (String, String, String)| {
        editing_id.set(Some(id));
        edit_date.set(date);
        new_content.set(content);
        show_add.set(true);
    };

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
                        p {
                            "Editando: "
                            input {
                                r#type: "date",
                                value: if editing_id().is_some() { "{edit_date}" } else { "{new_date}" },
                                oninput: move |evt| {
                                    if editing_id().is_some() {
                                        edit_date.set(evt.value());
                                    } else {
                                        new_date.set(evt.value());
                                    }
                                },
                            }
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
                        JournalEntry {
                            key: "{entry.base.id}",
                            entry,
                            on_edit: start_edit,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct JournalEntryProps {
    entry: crate::app_state::JournalEntry,
    on_edit: EventHandler<(String, String, String)>,
}

#[component]
fn JournalEntry(props: JournalEntryProps) -> Element {
    let entry_id_delete = props.entry.base.id.clone();
    let rendered = render_markdown(&props.entry.content);

    rsx! {
        div { class: "entry-card",
            div { class: "entry-header",
                span { class: "entry-date", "{props.entry.date}" }
                div { class: "entry-actions",
                    button {
                        onclick: move |_| {
                            props.on_edit.call((
                                props.entry.base.id.clone(),
                                props.entry.date.clone(),
                                props.entry.content.clone(),
                            ));
                        },
                        // Alternative: direct closure
                        // onclick: move |_| {} // keep for now
                        "Editar"
                    }
                    button {
                        class: "delete",
                        onclick: move |_| delete_journal_entry(&entry_id_delete),
                        "Eliminar"
                    }
                }
            }
            div { class: "entry-content markdown-body",
                dangerous_inner_html: "{rendered}",
            }
        }
    }
}
