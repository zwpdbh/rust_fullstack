#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[derive(Clone, Copy)]
struct IsLoggedIn(bool);

#[component]
pub fn DemoDynamicRendering() -> Element {
    use_context_provider(|| Signal::new(IsLoggedIn(false)));
    rsx!(
        div { class: "container",
            h1 { "Dynamic Rendering" }
            ul {
                li {
                    ConditionalRendering {
                        login: move |is_logged_in| {
                            info!("Do something for login, is_logged_in: {is_logged_in} ");
                        },
                        logout: move |is_logged_in| {
                            info!("Do something for logout, is_logged_in: {is_logged_in}");
                        }
                    }
                }
                li { RenderNothing {} }
                li { RenderingListV1 {} }
                li { RenderingListV2 {} }
            }
        }
    )
}

#[derive(PartialEq, Clone, Props)]
struct ConditionalRenderingProps {
    login: EventHandler<bool>,
    logout: EventHandler<bool>,
}

fn ConditionalRendering(prop: ConditionalRenderingProps) -> Element {
    // let log_in = is_logged_in.set(true);
    // let log_out = is_logged_in.set(false);
    let mut is_logged_in = use_context::<Signal<IsLoggedIn>>();
    rsx!(
        MyCard {
            h2 { "Conditional Rendering" }
            // We only render the welcome message if we are logged in
            // You can use if statements in the middle of a render block to conditionally render elements
            if is_logged_in().0 {
                // Notice the body of this if statement is rsx code, not an expression
                "Welcome!"
            }
            button {
                class: "button",
                // depending on the value of `is_logged_in`, we will call a different event handler
                onclick: move |_| {
                    if is_logged_in().0 {
                        prop.login.call(is_logged_in().0);
                        is_logged_in.write().0 = false;
                    } else {
                        prop.login.call(is_logged_in().0);
                        is_logged_in.write().0 = true;
                    }
                },
                if is_logged_in().0 {
                    // if we are logged in, the button should say "Log Out"
                    "Log Out"
                } else {
                    // if we are not logged in, the button should say "Log In"
                    "Log In"
                }
            }
        }
    )
}

#[component]
fn RenderNothing() -> Element {
    let mut is_logged_in = use_context::<Signal<IsLoggedIn>>();

    if is_logged_in().0 {
        return None;
    }

    rsx! {
        MyCard {
            h2 { "Rendering nothing" }
            label { class: "checkbox",
                "is_logged_in"
                input {
                    r#"type"#: "checkbox",
                    oninput: move |event| {
                        let logged_in = event.value() == "true";
                        is_logged_in.write().0 = logged_in;
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Clone)]
struct Comment {
    content: String,
    id: i32,
}

#[derive(PartialEq, Clone, Props)]
struct CommentComponentProp {
    comment: Comment,
}

#[component]
fn RenderingListV1() -> Element {
    let mut comment_field = use_signal(String::new);
    let mut next_id = use_signal(|| 0);
    let mut comments = use_signal(Vec::<Comment>::new);

    let comments_lock = comments.read();
    let comments_rendered = comments_lock.iter().map(|comment| {
        rsx! {
            CommentComponent { comment: comment.clone() }
        }
    });

    rsx! {
        MyCard {
            h2 { "Rendering list V1" }
            p { "Dioxus accepts iterators that produce Elements" }
            form {
                onsubmit: move |_| {
                    comments
                        .write()
                        .push(Comment {
                            content: comment_field(),
                            id: next_id(),
                        });
                    next_id += 1;
                    comment_field.set(String::new());
                },
                input {
                    value: "{comment_field}",
                    oninput: move |event| comment_field.set(event.value())
                }
                input { r#type: "submit" }
            }
            {comments_rendered}
        }
    }
}

#[component]
fn RenderingListV2() -> Element {
    let mut comment_field = use_signal(String::new);
    let mut next_id = use_signal(|| 0);
    let mut comments = use_signal(Vec::<Comment>::new);

    rsx! {
        MyCard {
            h2 { "Rendering list V2" }
            p {
                "Different from V1: Instead of using .iter, .map, and rsx, directly use 'for' loop with a body of rsx code."
            }
            form {
                onsubmit: move |_| {
                    comments
                        .write()
                        .push(Comment {
                            content: comment_field(),
                            id: next_id(),
                        });
                    next_id += 1;
                    comment_field.set(String::new());
                },
                input {
                    value: "{comment_field}",
                    oninput: move |event| comment_field.set(event.value())
                }
                input { r#type: "submit" }
            }
            for comment in comments() {
                // Notice the body of this for loop is rsx code, not an expression
                CommentComponent { comment }
            }
        }
    }
}

/// Because we annotate it as "#[component]".
/// We could pass prop.comment as regular parameters directly!
#[component]
fn CommentComponent(comment: Comment) -> Element {
    rsx!(
        MyCard {
            ul {
                li {
                    p { "comment: {comment.content}" }
                }
                li {
                    p { "id: {comment.id" }
                }
            }
        }
    )
}
