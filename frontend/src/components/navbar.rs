#![allow(non_snake_case)]
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx!(
        nav {
            "aria-label": "main navigation",
            role: "navigation",
            class: "navbar",

            div { class: "navbar-menu", id: "navbarBasicExample",
                div { class: "navbar-start",
                    Link { class: "navbar-item", to: Route::Home {}, "Home" }
                    Link { class: "navbar-item", to: Route::BlogList {},
                        {},
                        "Blog List"
                    }
                    Link { class: "navbar-item", to: Route::DemoMenuDefault {},
                        {},
                        "Demos"
                    }
                    div { class: "navbar-item has-dropdown is-hoverable",
                        a { class: "navbar-link", "More" }
                        div { class: "navbar-dropdown",
                            a { class: "navbar-item", "About" }
                            a { class: "navbar-item is-selected", "Jobs" }
                            a { class: "navbar-item", "Contact" }
                            hr { class: "navbar-divider" }
                            a { class: "navbar-item", "Report an issue" }
                        }
                    }
                }
                div { class: "navbar-end",
                    div { class: "navbar-item",
                        div { class: "buttons",
                            a { class: "button is-primary",
                                strong { "Sign up" }
                            }
                            a { class: "button is-light", "Log in" }
                        }
                    }
                }
            }
        }

        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
        Outlet::<Route> {}
    )
}
