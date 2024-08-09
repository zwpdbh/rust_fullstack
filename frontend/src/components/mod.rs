#![allow(non_snake_case)]
pub use blog::*;
pub use demos::async_with_coroutines::DemoCoroutines;
pub use demos::async_with_resource::DemoResource;
pub use demos::async_with_spawn::DemoSpawn;
pub use demos::context::DemoContext;
pub use demos::dynamic_rendering::DemoDynamicRendering;
pub use demos::event_handler::DemoEventHandler;
pub use demos::hooks::DemoHooks;
pub use demos::llm::DemoLLM;
pub use demos::prop::DemoProp;
pub use demos::rsx_basic::RsxBasic;
pub use demos::user_input::UserInput;
pub use demos::*;
use dioxus::prelude::*;
pub use navbar::NavBar;

mod blog;
mod demos;
mod navbar;

#[component]
pub fn Home() -> Element {
    rsx!(
        h1 { "Welcome to the Dioxus!" }
    )
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
