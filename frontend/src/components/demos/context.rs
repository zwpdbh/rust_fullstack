#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;
// use dioxus_logger::tracing::info;

#[derive(Clone, Copy)]
struct DarkMode(bool);

#[component]
pub fn DemoContext() -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));

    rsx!(
        h1 { "Demo context: sharing state" }
        p {
            "Often, multiple components need to access the same state. Depending on your needs, there are several ways to implement this."
        }
        ul {
            li { LiftingState {} }
            li { UseContext {} }
        }
    )
}

#[component]
fn LiftingState() -> Element {
    let container_style = r"
        display: flex;
        flex-direction: column;
        gap: 16px;
        margin: 0 auto;
        width: fit-content;
    ";

    let mut caption = use_signal(|| "me waiting for my rust code to compile".to_string());

    rsx!(
        MyCard {
            h2 { "Lifting State" }
            p {
                "One approach to share state between components is to "lift
                " it up to the nearest common ancestor."
            }
            p {
                "This means putting the `use_signal` hook in a parent component, and passing the needed values down as props."
            }

            div { style: "{container_style}",
                h1 { "Meme Editor" }
                Meme { caption }
                CaptionEditor { caption, oninput: move |event: FormEvent| caption.set(event.value()) }
            }
        }
    )
}

/// This component read and write signal
/// It use `oninput` to make changes
#[component]
fn CaptionEditor(caption: String, oninput: EventHandler<FormEvent>) -> Element {
    let input_style = r"
        border: none;
        background: cornflowerblue;
        padding: 8px 16px;
        margin: 0;
        border-radius: 4px;
        color: white;
    ";

    rsx! {
        MyCard {
            p { "caption editor" }
            input {
                style: "{input_style}",
                value: "{caption}",
                oninput: move |event| oninput.call(event)
            }
        }
    }
}

/// This component only read
#[component]
fn Meme(caption: String) -> Element {
    let container_style = r#"
        position: relative;
        width: fit-content;
    "#;

    let caption_container_style = r#"
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        padding: 16px 8px;
    "#;

    let caption_style = r"
        font-size: 32px;
        margin: 0;
        color: white;
        text-align: center;
    ";

    rsx! {
        MyCard {
            div { style: "{container_style}",
                img { src: "https://i.imgflip.com/2zh47r.jpg", height: "500px" }
                div { style: "{caption_container_style}",
                    p { style: "{caption_style}", "{caption}" }
                }
            }
        }
    }
}

#[component]
fn UseContext() -> Element {
    let mut dark_mode = use_context::<Signal<DarkMode>>();
    let style = if dark_mode().0 {
        "color:blue"
    } else {
        "color:write"
    };

    rsx!(
        MyCard {
            h2 { "Context" }
            p {
                "Sometimes, some state needs to be shared between multiple components far down the tree, and passing it down through props is very inconvenient. (prop drilling)"
            }
            p {
                "The use_context_provider hook provides any Clone context (including Signals!) to any child components."
            }
            p {
                "Child components can use the use_context hook to get that context and if it is a Signal, they can read and write to it."
            }

            label { style: "{style}", class: "checkbox",
                "Dark Mode"
                input {
                    r#type: "checkbox",
                    oninput: move |event| {
                        let is_enabled = event.value() == "true";
                        dark_mode.write().0 = is_enabled;
                    }
                }
            }
        }
    )
}
