#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::Icon;
use dioxus_logger::tracing::info;

/// Example from: https://github.com/fairjm/dioxus-openai-qa-gui
#[component]
pub fn UserInput() -> Element {
    // let mut setting_hide = use_signal(|| true);

    rsx!(
        div { class: "container",
            ul {
                li { ControlledInput {} }

                li { UncontrolledInput {} }

                li { HandleFile {} }
            }
        }
    )
}

#[component]
fn ControlledInput() -> Element {
    let mut name = use_signal(|| "bob".to_string());

    rsx!(
        MyCard {
            h1 { "Controlled Inputs" }
            p { "With controlled inputs, you are directly in charge of the state of the input. " }
            p { "This gives you a lot of flexibility, and makes it easy to keep things in sync. " }

            input {
                // we tell the component what to render
                value: "{name}",
                // and what to do when the value changes
                oninput: move |event| name.set(event.value())
            }

            p { "Your input is: {name}" }
        }
    )
}

#[component]
fn UncontrolledInput() -> Element {
    rsx! {
        MyCard {
            h2 { "Uncontrolled Inputs" }
            form {
                onsubmit: move |event| {
                    info!("Submitted! {event:?}");
                },
                input { name: "name" }
                input { name: "age" }
                input { name: "date" }
                input { r#type: "submit" }
            }
        }
    }
}

#[component]
fn HandleFile() -> Element {
    let mut files_uploaded: Signal<Vec<String>> = use_signal(Vec::new);
    rsx!(
        MyCard {
            h2 { "Handling files" }
            p {
                "Notice: type is a Rust keyword, so when specifying the type of the input field, you have to write it as r#type:"file
                "."
            }
            input {
                // tell the input to pick a file
                r#type: "file",
                // list the accepted extensions
                accept: ".txt,.rs",
                // To select a folder, we need to set it to true
                directory: false,
                // pick multiple files
                multiple: true,
                onchange: move |evt| {
                    async move {
                        if let Some(file_engine) = evt.files() {
                            let files = file_engine.files();
                            for file_name in &files {
                                if let Some(file) = file_engine.read_file_to_string(file_name).await
                                {
                                    files_uploaded.write().push(file);
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}

#[allow(unused)]
#[component]
fn Nav() -> Element {
    rsx! {
        nav { class: "level mt-2 mb-2",
            div { class: "level-left",
                div { class: "level-item",
                    p { class: "title is-size-4 has-text-centered", "LLA test" }
                }
                div { class: "level-item",
                    a {
                        class: "button is-small",
                        target: "_blank",
                        href: "https://github.com/fairjm/dioxus-openai-qa-gui",
                        span { class: "icon is-small",
                            Icon { width: 24, height: 24, fill: "#6e7781", icon: FaGithub }
                        }
                        span { "GitHub" }
                    }
                }
            }
        }
    }
}
