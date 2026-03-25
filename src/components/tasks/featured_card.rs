use dioxus::prelude::*;

#[component]
pub fn FeaturedCard(on_action: EventHandler<()>) -> Element {
    rsx! {
        section { class: "featured-card",
            div { class: "featured-card-content",
                h2 { "Garden Journal Entry" }
                p { "Document the flowering of the Bird of Paradise. It's the first bloom of the season!" }
                button {
                    class: "featured-btn",
                    onclick: move |_| on_action.call(()),
                    "Start Journaling"
                }
            }

            div { class: "featured-card-image",
                img {
                    src: "https://lh3.googleusercontent.com/aida-public/AB6AXuB1TdL4vjVes0C9qStLBuxL6eVHkNT3p3CKqu0lRz0mqvNj6WnOr4Al_xGUkD6bd-vag6exrDie6EfBjxlPVXUaDRFrnAUU8v5gEd3TprLZh5bTQS4uoHkx9gk-roxn6b18lz62LwYCfVQRIzjcb5Lx3fTmAcbyl7osM-S4J97xHwrAKQyUcczeiAvOLzSicXMOKJGObXOT8pVZ9ywuC0Ddb9QV2_0buUwdrAajFWYjJNxiiEZP26DeRTtoRPAxGGLai84LI9DK72cz",
                    alt: "Bird of Paradise flower"
                }
            }
        }
    }
}
