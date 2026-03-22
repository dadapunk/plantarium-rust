use dioxus::prelude::*;

#[component]
pub fn ProTip() -> Element {
    rsx! {
        div { class: "pro-tip-card",
            h4 { "💡 Pro Tip" }
            h5 { "Companion Planting for Spring" }
            p {
                "Planting basil next to your tomatoes can improve their flavor
                and repel pests naturally."
            }
            a { href: "#", class: "read-guide", "📖 Read Guide →" }
        }
    }
}
