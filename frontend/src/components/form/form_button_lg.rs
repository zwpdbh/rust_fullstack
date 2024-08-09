use dioxus::{events::MouseEvent, prelude::*};

#[derive(Clone, Props, PartialEq)]
pub struct FormButtonProps {
    onclick: EventHandler<MouseEvent>,
    label: String,
}

#[component]
pub fn FormButton_Lg(props: FormButtonProps) -> Element {
    rsx!(
        button {
            class: "btn btn-lg btn-primary pull-xs-right",
            r#type: "button",
            onclick: move |event| { props.onclick.call(event) },
            "{props.label}"
        }
    )
}
