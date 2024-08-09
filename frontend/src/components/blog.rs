#![allow(non_snake_case)]
use crate::routes::Route;
use dioxus::prelude::*;

/// Place holder for Blog section
#[component]
pub fn Blog() -> Element {
    rsx! {
        h1 { "Blog Detail" }
        Outlet::<Route> {}
    }
}

#[component]
pub fn BlogList() -> Element {
    rsx! {
        h2 { "Choose a post" }
        ul {
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 1".into(),
                    },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 2".into(),
                    },
                    "Read the second blog post"
                }
            }
        }
    }
}

// The name prop comes from the /:name route segment
#[component]
pub fn BlogPost(name: String) -> Element {
    rsx! {
        h2 { "Blog Post: {name}" }
    }
}
