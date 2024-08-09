#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use gloo_timers::future::sleep;
use std::time::Duration;

#[component]
pub fn DemoCoroutines() -> Element {
    rsx!(
        h1 { "Coroutines" }
        p {
            "It allows you to write asynchronous code that can yield values over time, suitable for tasks like WebSocket polling, background timers, and other periodic actions."
        }
        ul {
            li { SimpleCoroutine {} }
        }
    )
}

enum CounterMsg {
    KeepInc(usize),
    Reset,
}

/// Observation:
/// Click 3 time on Inc button then click Reset button.
/// The counter changed to 1, 2, 3 and to 0 with interval 3 seconds.
#[component]
fn SimpleCoroutine() -> Element {
    let mut counter = use_signal(|| 0);

    let update_counter = use_coroutine(|mut rx: UnboundedReceiver<CounterMsg>| async move {
        while let Some(msg) = rx.next().await {
            // simulate get result from api
            sleep(Duration::from_secs(3)).await;
            match msg {
                CounterMsg::KeepInc(n) => {
                    counter += n;
                }
                CounterMsg::Reset => {
                    counter.set(0);
                }
            }
        }
    });
    rsx! {
        MyCard {
            div {
                button {
                    class: "button",
                    onclick: move |_| { update_counter.send(CounterMsg::KeepInc(1)) },
                    "Inc"
                }
                button {
                    class: "button",
                    onclick: move |_| { update_counter.send(CounterMsg::Reset) },
                    "Reset"
                }

                p { "counter: {counter}" }
            }
        }
    }
}
