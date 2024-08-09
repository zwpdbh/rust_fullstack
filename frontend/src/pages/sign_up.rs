use crate::components::{FormButton_Lg, FormInput_Lg};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[allow(unused)]
pub fn SignUpPage() -> Element {
    let mut name = use_context::<Signal<String>>();
    let email = use_context::<Signal<String>>();
    let password = use_context::<Signal<String>>();

    rsx!(
        div { class: "auth-page",
            div { class: "container page",
                div { class: "row",
                    div { class: "col-md-6 offset-md-3 col-xs-12" }
                    h1 { class: "text-xs-center", "Sign up" }
                    p { class: "text-xs-center", "Sign in if you have an account" }
                    br {}
                    br {}

                    ul { class: "error-messages",
                        li { "That email is already taken" }
                    }
                    form {
                        FormInput_Lg {
                            oninput: move |event: Event<FormData>| name.set(event.value()),
                            placeholder: "You Name".to_string()
                        }
                        FormButton_Lg {
                            onclick: move |_: MouseEvent| {
                                info!(":: SignUpPage] button clicked. name: {} | email: {}", name, email);
                            },
                            label: "Sign up".to_string()
                        }
                    }
                }
            }
        }
    )
}
