use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatCardProps {
    pub title: String,
    pub value: String,
    pub icon: String,
}

#[component]
pub fn StatCard(props: StatCardProps) -> Element {
    rsx! {
        div { class: "stat-card",
            div { class: "stat-icon",
                // Icon placeholder - will be replaced with dioxus-icons
                span { "{props.icon}" }
            }
            div { class: "stat-content",
                h3 { "{props.title}" }
                p { class: "stat-value", "{props.value}" }
            }
        }
    }
}
