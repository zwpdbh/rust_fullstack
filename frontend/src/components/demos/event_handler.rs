#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use std::fmt;

#[component]
pub fn DemoEventHandler() -> Element {
    rsx!(
        div { class: "container",
            h1 { "DemoEventHandler" }
            ul {
                li { MyCard {
                    div {
                        h2 { "Simple event handler" }
                        button { onclick: move |event| info!("Clicked! Event: {event:?}"),
                            "click me!"
                        }
                    }
                } }

                li { MyCard {
                    div {
                        h2 { "Event propagation" }
                        p {
                            "a click event on a button inside a div would first trigger the button's event listener and then the div's event listener."
                        }
                        p {
                            "If you want to prevent this behavior, you can call stop_propagation() on the event:"
                        }
                        div { onclick: move |_event| {},
                            "outer"
                            MyCard {
                                button {
                                    onclick: move |event| {
                                        event.stop_propagation();
                                    },
                                    "inner"
                                }
                            }
                        }
                    }
                } }

                li { MyCard {
                    h2 { "Prevent Default event" }
                    p {
                        "add the prevent_default attribute with the name of the handler whose default behavior you want to stop."
                    }
                    a {
                        href: "https://example.com",
                        prevent_default: "onclick",
                        onclick: |_| info!("link clicked"),
                        "example.com"
                    }
                } }

                li { MyCard {
                    h2 { "Handler Props" }
                    p { "Sometimes, you might want to make a component that accepts an event handler" }

                    FancyButton { onclick: move |event| info!("Clicked! {event:?}") }

                    p { "use spawn to do async operations:" }
                    FancyButton {
                        onclick: move |event| {
                            spawn(async move {
                                println!("Clicked! {event:?}");
                            });
                        }
                    }

                    h2 { "Custom data" }
                    p {
                        "Event Handlers are generic over any type, so you can pass in any data you want to them, e.g:"
                    }
                    CustomFancyButton { onclick: move |x| { info!("x is {:?}", x) } }
                } }
            }
        }
    )
}

#[derive(PartialEq, Clone, Props)]
struct FancyButtonProps {
    onclick: EventHandler<MouseEvent>,
}

fn FancyButton(props: FancyButtonProps) -> Element {
    rsx! {
        button { onclick: move |evt| props.onclick.call(evt), "click me pls." }
    }
}

#[derive(Debug)]
struct ComplexData(i32);

impl fmt::Display for ComplexData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = format!("ComplexData({})", self.0);
        write!(f, "{}", output)
    }
}

#[derive(PartialEq, Clone, Props)]
struct CustomFancyButtonProps {
    onclick: EventHandler<ComplexData>,
}

fn CustomFancyButton(props: CustomFancyButtonProps) -> Element {
    rsx! {
        button { onclick: move |_| props.onclick.call(ComplexData(0)), "CustomFancyButton" }
    }
}
