#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;

#[component]
pub fn RsxBasic() -> Element {
    rsx!(
        ul {
            li { DemoAttributes {} }
            li { DemoPassHtmlDirectly {} }
            li { ConditionalRender {} }
            li { DynamicText {} }
            li { Interpolation {} }
            li { Expression {} }
            li { Loop {} }
        }
    )
}

#[component]
fn DemoAttributes() -> Element {
    let large_font = true;
    rsx! {
        MyCard { 
            div {
                h1 { "Conditional Attributes" }
                div { class: if large_font { "text-xl" }, "Hello, World!" }

                h1 { "Attribute 'type'" }
                div {
                    input { r#type: "text", color: "red" }
                }

                h1 { "Custom attribute" }
                p {
                    "Dioxus has a pre-configured set of attributes that you can use. RSX is validated at compile time to make sure you didn't specify an invalid attribute. If you want to override this behavior with a custom attribute name, specify the attribute in quotes:"
                }
                div { "style": "width: 20px; height: 20px; background-color: red;" }
            }
        }
    }
}

#[component]
fn DemoPassHtmlDirectly() -> Element {
    // this should come from a trusted source
    let contents = "live <b>dangerously</b>";

    rsx! {
        MyCard { 
            h1 { "DemoPassHtmlDirectly" }
            div { dangerous_inner_html: "{contents}" }
        }
    }
}

#[component]
pub fn ConditionalRender() -> Element {
    let is_logged_in: Option<i32> = None;
    rsx! {
        MyCard { 
            h1 { "ConditionalRender" }
            // We only render the welcome message if we are logged in
            // You can use if statements in the middle of a render block to conditionally render elements
            match is_logged_in {
                Some(_) => {
                    // if we are logged in, the button should say "Log Out"
                    "Log Out"
                }
                None => {
            // if we are not logged in, the button should say "Log In"
                    "Log In"
                }
            }
        }
    }
}

#[component]
fn DynamicText() -> Element {
    let title = "title";
    let by = "author";
    let score = 0;
    let time = chrono::Utc::now();
    let comments = "comments";

    rsx! {

        MyCard { 
            h1 { "DynamicText" }
            "{title} by {by} ({score}) {time} {comments}"
        }
    }
}

#[component]
fn Interpolation() -> Element {
    let coordinates = (42, 0);
    let country = "es";
    rsx! {
        MyCard { 
            h1 { "Interpolation" }
            div {
                class: "country-{country}",
                left: "{coordinates.0:?}",
                top: "{coordinates.1:?}",
                // arbitrary expressions are allowed,
                // as long as they don't contain `{}`
                div { "{country.to_uppercase()}" }
                div { "{7*6}" }
                // {} can be escaped with {{}}
                div { "{{}}" }
            }
        }
    }
}

#[component]
fn Expression() -> Element {
    let text = "Dioxus";

    rsx! {
        MyCard { 
            h1 { "Expression" }
            span {
                {text.to_uppercase()},
                {(0..10).map(|i| rsx!{ "{i}" })}
            }
        }
    }
}

#[component]
fn Loop() -> Element {
    rsx! {
        MyCard { 
            h1 { "Loop" }
            // use a for loop where the body itself is RSX
            div {
                // create a list of text from 0 to 9
                for i in 0..3 {
                    // NOTE: the body of the loop is RSX not a rust statement
                    div { "{i}" }
                }
            }
            // iterator equivalent
            div { {(0..3).map(|i| rsx!{ div { "{i}" } })} }
        }
    }
}
