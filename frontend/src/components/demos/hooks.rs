#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;

#[component]
pub fn DemoHooks() -> Element {
    rsx! {
        h1 { "Demo hooks" }
        p { "Hooks allow us to create state in our components. " }
        p {
            "Hooks are Rust functions you call in a constant order in a component that add additional functionality to the component."
        }

        ul {
            li { MyCard {
                h2 { "use_signal hook" }
                p { "It is one of the simplest hooks." }

                Counter {}
            } }

            li { MyCard {
                h2 { "You can use multiple hooks in the same component if you want" }
                MultipleHooksCounter {}
            } }
        }
    }
}

#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    rsx!(
        p { "High-Five counter: {count}" }
        button { class: "button", onclick: move |_| count += 1, "Up " }
        button { class: "button", onclick: move |_| count -= 1, "Down " }
    )
}

#[component]
fn MultipleHooksCounter() -> Element {
    let mut count_a = use_signal(|| 0);
    let mut count_b = use_signal(|| 0);

    rsx! {
        p { "Counter_a: {count_a}" }
        button { class: "button", onclick: move |_| count_a += 1, "a++" }
        button { class: "button", onclick: move |_| count_a -= 1, "a--" }
        p { "Counter_b: {count_b}" }
        button { class: "button", onclick: move |_| count_b += 1, "b++" }
        button { class: "button", onclick: move |_| count_b -= 1, "b--" }
    }
}
