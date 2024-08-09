use crate::{commons::AppState, routes::Route};
use dioxus::prelude::*;

#[allow(unused)]
pub fn SignOutPage() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    app_state.write().token = None;

    navigator.push(Route::Home {});
    rsx!(
        div {
            h2 { "Signed out" }
        }
    )
}
