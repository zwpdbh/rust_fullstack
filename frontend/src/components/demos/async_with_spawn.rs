#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn DemoSpawn() -> Element {
    let mut response = use_signal(|| String::from("..."));

    let log_in = move |_| {
        spawn(async move {
            let resp = reqwest::Client::new()
                .get("https://dioxuslabs.com")
                .send()
                .await;

            match resp {
                Ok(_data) => {
                    info!("dioxuslabs.com responded!");
                    response.set("dioxuslabs.com responded!".into());
                }
                Err(err) => {
                    info!("Request failed with error: {err:?}")
                }
            }
        });
    };

    rsx!(
        h1 { "Demo spawn" }
        p {
            "It is suitable to response to an event where 'use_resource' and 'use_coroutine' are useful if you want to unconditionally spawn the future."
        }

        MyCard {
            button { class: "button", onclick: log_in, "Response: {response}" }
        }
    )
}
