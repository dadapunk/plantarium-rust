use crate::app_state::{create_event, delete_event, CalendarEvent, TaskType, EVENTS};
use crate::components::Navbar;
use chrono::Datelike;
use dioxus::prelude::*;

#[component]
pub fn Calendar() -> Element {
    let mut current_month = use_signal(|| chrono::Local::now().month() as i32);
    let mut current_year = use_signal(|| chrono::Local::now().year());
    let mut show_add = use_signal(|| false);
    let mut new_title = use_signal(|| String::new());
    let mut new_date = use_signal(|| String::new());
    let mut new_type = use_signal(|| TaskType::Custom);

    let all_events = EVENTS.read();
    let events_for_month: Vec<CalendarEvent> = all_events
        .iter()
        .filter(|e| e.base.deleted_at.is_none())
        .filter(|e| {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(&e.date, "%Y-%m-%d") {
                date.month() as i32 == current_month() && date.year() == current_year()
            } else {
                false
            }
        })
        .cloned()
        .collect();

    let month_names = [
        "Enero",
        "Febrero",
        "Marzo",
        "Abril",
        "Mayo",
        "Junio",
        "Julio",
        "Agosto",
        "Septiembre",
        "Octubre",
        "Noviembre",
        "Diciembre",
    ];
    let day_names = ["Dom", "Lun", "Mar", "Mié", "Jue", "Vie", "Sáb"];

    let days_in_month = match current_month() {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if current_year() % 4 == 0 {
                29
            } else {
                28
            }
        }
        _ => 30,
    };

    let cm = current_month();
    let cy = current_year();
    let day_cells: Vec<_> = (1..=days_in_month)
        .map(|day| {
            let _date_str = format!("{}-{:02}-{:02}", cy, cm, day);
            let day_events: Vec<_> = events_for_month
                .iter()
                .filter(|e| {
                    if let Ok(d) = chrono::NaiveDate::parse_from_str(&e.date, "%Y-%m-%d") {
                        d.day() == day as u32
                    } else {
                        false
                    }
                })
                .cloned()
                .collect();
            (day, day_events)
        })
        .collect();

    rsx! {
        div { class: "app-container",
            Navbar {}
            div { class: "main-content",
                div { class: "header",
                    h1 { "Calendario" }
                    button {
                        class: "add-btn",
                        onclick: move |_| show_add.toggle(),
                        if show_add() { "Cancelar" } else { "+ Añadir Evento" }
                    }
                }

                if show_add() {
                    div { class: "add-form",
                        input {
                            r#type: "text",
                            placeholder: "Título del evento",
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
                        button {
                            onclick: move |_| {
                                if !new_title().trim().is_empty() && !new_date().is_empty() {
                                    create_event(&new_title(), &new_date(), new_type(), None);
                                    new_title.set(String::new());
                                    new_date.set(String::new());
                                    show_add.set(false);
                                }
                            },
                            "Guardar"
                        }
                    }
                }

                div { class: "calendar",
                    div { class: "calendar-header",
                        button {
                            onclick: move |_| {
                                let m = current_month() - 1;
                                if m < 1 {
                                    current_month.set(12);
                                    current_year.set(current_year() - 1);
                                } else {
                                    current_month.set(m);
                                }
                            },
                            "←"
                        }
                        h2 { "{month_names[(current_month() - 1) as usize]} {current_year()}" }
                        button {
                            onclick: move |_| {
                                let m = current_month() + 1;
                                if m > 12 {
                                    current_month.set(1);
                                    current_year.set(current_year() + 1);
                                } else {
                                    current_month.set(m);
                                }
                            },
                            "→"
                        }
                    }

                    div { class: "weekdays",
                        for day in day_names.iter() {
                            div { class: "weekday", "{day}" }
                        }
                    }

                    div { class: "days",
                        for (day, day_events) in &day_cells {
                            div { class: "day",
                                span { class: "day-number", "{day}" }
                                div { class: "day-events",
                                    for event in day_events {
                                        div { class: "day-event",
                                            span { "{event.title}" }
                                            button {
                                                class: "event-delete",
                                                onclick: { let eid = event.base.id.clone(); move |_| delete_event(&eid) },
                                                "✕"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
