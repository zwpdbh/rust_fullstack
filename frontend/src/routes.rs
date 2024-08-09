#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::*;

#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    // All routes under the NavBar layout will be rendered inside of the NavBar Outlet
    #[layout(NavBar)]
        #[route("/")]
        Home {},

        // For blog section
        #[nest("/blog")]
            #[layout(Blog)]
                #[route("/")]
                BlogList {},

                #[route("/post/:name")]
                BlogPost { name: String },
            #[end_layout]
        #[end_nest]

        // For demo section 
        #[nest("/demo")]
            #[layout(Demo)]
                #[route("/")]
                DemoMenuDefault {},

                #[route("/rsxbasic")]
                RsxBasic {},

                #[route("/prop")]
                DemoProp {},

                #[route("/event_handler")]
                DemoEventHandler {},

                #[route("/hoooks")]
                DemoHooks {},

                #[route("/userinput")]
                UserInput {},

                #[route("/context")]
                DemoContext {},

                #[route("/dynamicrendering")]
                DemoDynamicRendering {},

                #[route("/demo_resource")]
                DemoResource {},

                #[route("/demo_coroutines")]
                DemoCoroutines {},

                #[route("/demo_spawn")]
                DemoSpawn {},

                #[route("/llm")]
                DemoLLM {},
            #[end_layout]
        #[end_nest]
    #[end_layout]

    // This will redirect user to /blog or /blog/post/:name 
    #[nest("/myblog")]
        #[redirect("/", || Route::BlogList {})]
        #[redirect("/:name", |name: String| Route::BlogPost { name })]
    #[end_nest]

    // #[route("/acstor")]
    // Acstor {},
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
