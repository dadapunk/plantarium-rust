use crate::components::tasks::{
    CalendarWidget, FeaturedCard, FertilizerAlertCard, NewTask, TaskCard, TaskCardData,
    TaskItemData, TaskListItem, TaskModal, TasksHeader,
};
use crate::components::DashboardHeader;
use dioxus::prelude::*;

#[component]
pub fn Tasks() -> Element {
    let mut show_modal = use_signal(|| false);

    let urgent_tasks = use_signal(|| {
        vec![
        TaskCardData {
            icon: "water_drop".to_string(),
            badge: "Hydration".to_string(),
            title: "Water Fiddle Leaf Fig".to_string(),
            description: "Soil is extremely dry. Check drainage after watering.".to_string(),
            plant_image: "https://lh3.googleusercontent.com/aida-public/AB6AXuDGcbqdNSTMM4MQp5oGZ9gRe0DLuIEmQa9qqGUPq9KpUM-iI71CbBB7neAiUfSqCEBIO_K6-G7xvY3gCm1MEzyhioFT8N5gsM-cvyO2QhHQ6PVDU-kv_tS3UcYg47DZ2ryQJh0NFjXTGuXcdx817tyDqF6oVY0cVTYGd7o_ZUIC7-rR3aTvSRqgGZD-aJH8pLTTd0AExHLYplXMQVQs4LKNhupYlO78v70HONEY9vOt3N_RoWhyqlIHnUa5nk3C-wKbY3dGH4wAtx7".to_string(),
            location: "conservatory_west_wing".to_string(),
            date: "Today".to_string(),
        },
        TaskCardData {
            icon: "pest_control".to_string(),
            badge: "Health".to_string(),
            title: "Pest Inspection".to_string(),
            description: "Possible mealybugs detected on the Swiss Cheese plant.".to_string(),
            plant_image: "https://lh3.googleusercontent.com/aida-public/AB6AXuCHA3Kwqr7dL2NE66nQUXclAJ-b4SR8t1te5G1BF9PyRAmsokVmN2cS5O9PwUm3kL9Ib38usB0qeHY29CL2CK_89VaB6QtQcA332h5ZatTPsxUQQze52VXADJc4H_a5XRyrQGPBdEhstFYNYxsoVL55KHmuKdL-MiL5wUHmoCl6oE5_JPXgfK3iKt0UBA2fwgBx1GpR_sYEAwvqL1SgNV1VafFQBvOr3i4T6lnvPg3upHXfSVnRGpMH4rSKoZeMtSAysCOnYMsvWT4F".to_string(),
            location: "terrace_section_04".to_string(),
            date: "11:00 AM".to_string(),
        },
    ]
    });

    let routine_tasks = use_signal(|| {
        vec![
        TaskItemData {
            title: "Mist Tropical Ferns".to_string(),
            description: "Keep humidity above 60% during the dry afternoon.".to_string(),
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuDSw4aav9Ict1I9_7GPlSBWa3SKahIctm_qjZz1oiv4lmW3shoCmiYRsBpOE850FYJ2Xmq5tYqWy2Y_O_ez20ZoKykU-0I8a8z57y6QLmgEmcnidSBj9uOdccWfDBvMZ01M_pTVJzR9zEl7_nnmixmru1aqZ8trAdbz-AzBt3JLwATeAqSsFh9FtVxM8GwsvSobZm7Fkyyw47p2Fi-94pzs8y3Rm5Yq-PmPqAvY7yI6YbEmwvWrKFhvHurMtrfuO-dP0jsxKgHUPWQA".to_string(),
            frequency: "Every Day".to_string(),
            frequency_icon: "repeat".to_string(),
            badge: "Pruning".to_string(),
        },
        TaskItemData {
            title: "Soil Aeration".to_string(),
            description: "Gently poke holes in the topsoil of the large palms.".to_string(),
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuDYIFhDuUtdRFNm9Gs2MwJKvF7fzC08hPtPbH4EyhzMjdBpYLjVBWiSA0GAwv67KnDTijckL73bthXEVt5hu4BWtg9UHl3RTkvPLNyILJTs_vN7CHnpKe9HGl-gvCcc40kFSQF5df4TM1CKrkRUoD0WToWSuKmwG-hpl40hx8GX8YIWh1P-AN3cZ4LZcbCxxj8vT70En8wTufwiztmAkkMRctQrJz0MBGHLFGv1cZ2PJXAQveFc97CC2RP4BiOFuXdwJO89Yyv1gf3Y".to_string(),
            frequency: "Weekly".to_string(),
            frequency_icon: "calendar_today".to_string(),
            badge: "Tools".to_string(),
        },
        TaskItemData {
            title: "Rotate Succulents".to_string(),
            description: "Ensure even light exposure for the windowsill collection.".to_string(),
            image: "https://lh3.googleusercontent.com/aida-public/AB6AXuCWM9OmL4XxA0ZiNRgM1ISU2pahwLuUc1Gyu6R0nF9NtWkTV5PVTtWvF4fxDRUnIoSTuYwj0oZ9ViVW8g4WF_FuNufT2pDSxz2J6ZaZR6w7V5XfFa23NIqx1nC2312wdR1Y7EP4f3gX2FS3y1y0bVjenjDXPpIeYQN8KeboWbNTwbWlxiad1d5BjBcz3_8Q0NEMKMNweShdp5w4fNMEa0sbzvoJTk6lhR10F5dR75KmRraluI2Q4OHqYGDmEUyNZXA7punCpCiZxWDZ".to_string(),
            frequency: "Bi-Weekly".to_string(),
            frequency_icon: "rotate_right".to_string(),
            badge: "Light".to_string(),
        },
    ]
    });

    let handle_add_task = move |_new_task: NewTask| {
        show_modal.set(false);
    };

    rsx! {
        div { class: "tasks-page",
            DashboardHeader {}

            main { class: "tasks-main",
                TasksHeader {
                    on_add_click: move |_| show_modal.set(true),
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
                        // Urgent Care Section
                        section { class: "task-category",
                            div { class: "task-category-header",
                                h2 { class: "task-category-title", "Urgent Care" }
                                span { class: "task-category-count urgent", "3 Tasks" }
                            }

                            div { class: "task-cards-grid",
                                for task in urgent_tasks() {
                                    TaskCard {
                                        task: task.clone(),
                                        on_toggle: move |_| {},
                                    }
                                }
                            }
                        }

                        // Routine Maintenance Section
                        section { class: "task-category",
                            div { class: "task-category-header",
                                h2 { class: "task-category-title", "Routine Maintenance" }
                                span { class: "task-category-count routine", "8 Tasks" }
                            }

                            div { class: "task-list",
                                for task in routine_tasks() {
                                    TaskListItem {
                                        task: task.clone(),
                                        on_toggle: move |_| {},
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
