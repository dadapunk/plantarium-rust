use dioxus::prelude::*;

#[component]
pub fn DashboardHeader() -> Element {
    let mut mobile_menu_open = use_signal(|| false);

    rsx! {
        header { class: "dashboard-header-nav",
            div { class: "dashboard-header-nav-inner",
                // Brand
                div { class: "dashboard-brand", "Plantarium" }

                // Desktop Navigation
                nav { class: "dashboard-nav",
                    a { href: "#", class: "active", "Dashboard" }
                    a { href: "#", "Tasks" }
                    a { href: "#", "Journal" }
                    a { href: "#", "Library" }
                }

                // Actions Area
                div { class: "dashboard-actions",
                    // Weather Widget (hidden on mobile)
                    div { class: "weather-widget",
                        span { class: "weather-widget-icon", "☀" }
                        div { class: "weather-widget-content",
                            span { class: "weather-widget-season", "Summer Solstice" }
                            span { class: "weather-widget-temp", "24°C" }
                        }
                    }

                    // Icon Buttons
                    button { class: "icon-btn", "🔔" }
                    button { class: "icon-btn", "⚙" }

                    // User Avatar
                    div { class: "user-avatar",
                        div {
                            style: "width: 100%; height: 100%; background: var(--surface-container-high); display: flex; align-items: center; justify-content: center; color: var(--outline);",
                            "👤"
                        }
                    }

                    // Hamburger Button (mobile only)
                    button {
                        class: "hamburger-btn",
                        class: if mobile_menu_open() { "active" } else { "" },
                        onclick: move |_| {
                            mobile_menu_open.set(!mobile_menu_open());
                        },
                        span { class: "hamburger-line" }
                        span { class: "hamburger-line" }
                        span { class: "hamburger-line" }
                    }
                }
            }

            // Mobile Menu Dropdown
            if mobile_menu_open() {
                nav { class: "mobile-nav",
                    a { href: "#", class: "active", "Dashboard" }
                    a { href: "#", "Tasks" }
                    a { href: "#", "Journal" }
                    a { href: "#", "Library" }

                    // Weather in mobile menu
                    div { class: "mobile-weather",
                        span { "☀ Summer Solstice • 24°C" }
                    }
                }
            }
        }
    }
}
