use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct FormInputProps {
    // Notice: pub type FormEvent = Event<FormData>
    oninput: EventHandler<FormEvent>,
    placeholder: Option<String>,
    value: Option<String>,
}

#[component]
pub fn FormInput_Lg(props: FormInputProps) -> Element {
    let placeholder = props.placeholder.clone().unwrap_or_default();
    let value = props.placeholder.clone().unwrap_or_default();

    rsx!(
        fieldset { class: "form-group",
            input {
                class: "form-control form-control-lg",
                r#type: "text",
                oninput: move |evt| { props.oninput.call(evt) },
                placeholder: "{placeholder}",
                value: "{value}"
            }
        }
    )
}
