use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AddGardenCardProps {
    pub on_click: EventHandler<()>,
}

#[component]
pub fn AddGardenCard(props: AddGardenCardProps) -> Element {
    rsx! {
        div {
            class: "add-garden-card",
            onclick: move |_| props.on_click.call(()),

            div { class: "add-icon",
                span { "➕" }
            }

            h3 { "Add New Garden" }
            p { "Define a new planting area and start tracking your growth." }
        }
    }
}
