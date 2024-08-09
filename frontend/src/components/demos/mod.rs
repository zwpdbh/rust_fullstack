#![allow(non_snake_case)]
use crate::routes::Route;
use dioxus::prelude::*;

pub mod async_with_coroutines;
pub mod async_with_resource;
pub mod async_with_spawn;
pub mod context;
pub mod dynamic_rendering;
pub mod event_handler;
pub mod hooks;
pub mod llm;
pub mod prop;
pub mod rsx_basic;
pub mod user_input;

/// Place holder for Demo section
#[component]
pub fn Demo() -> Element {
    rsx!(
        div { class: "columns",
            div { class: "column is-one-fifth", DemoMenu {} }
            div { class: "column", Outlet::<Route> {} }
        }
    )
}

#[component]
pub fn DemoMenuDefault() -> Element {
    rsx!()
}

/// This is the sidebar menu to show different demos for demo section
#[component]
fn DemoMenu() -> Element {
    rsx!(
        aside { class: "menu",
            p { class: "menu-label", "General" }
            ul { class: "menu-list",
                li {
                    Link { to: Route::RsxBasic {}, "RsxBasic" }
                }
                li {
                    Link { to: Route::DemoProp {}, "Prop" }
                }
                li {
                    Link { to: Route::DemoEventHandler {}, "Event Handler" }
                }
                li {
                    Link { to: Route::DemoHooks {}, "Hooks" }
                }
                li {
                    Link { to: Route::UserInput {}, "User Input" }
                }
                li {
                    Link { to: Route::DemoContext {}, "Context" }
                }
                li {
                    Link { to: Route::DemoDynamicRendering {}, "Dynamic Rendering" }
                }
                li {
                    Link { to: Route::DemoResource {}, "Async with Resource" }
                }
                li {
                    Link { to: Route::DemoCoroutines {}, "Async with Coroutines" }
                }
                li {
                    Link { to: Route::DemoSpawn {}, "Async with Spawn" }
                }
            }
            p { class: "menu-label", "LLM service" }
            ul { class: "menu-list",
                li {
                    Link { to: Route::DemoLLM {}, "LLM service" }
                }
            }
            p { class: "menu-label", "ACStor CRUD" }
            ul { class: "menu-list",
                li {
                    a { "Team Settings" }
                }
                li {
                    a { "Manage Your Team" }
                    ul {
                        li {
                            a { "Members" }
                        }
                        li {
                            a { "Plugins" }
                        }
                        li {
                            a { "Add a member" }
                        }
                    }
                }
                li {
                    a { "Invitations" }
                }
                li {
                    a { "Cloud Storage Environment Settings" }
                }
                li {
                    a { "Authentication" }
                }
            }
        }
    )
}

#[component]
fn MyCard(children: Element) -> Element {
    rsx!(
        div { class: "card",
            div { class: "card-content",
                div { class: "content", { children } }
            }
        }
    )
}
