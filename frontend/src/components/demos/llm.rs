#![allow(non_snake_case)]
#[allow(unused)]
use super::MyCard;
use crate::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
// use futures_util::io::Sink;

use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::Icon;
#[allow(unused)]
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
#[allow(unused)]
use serde_json::json;
#[allow(unused)]
use serde_json::Value;

#[component]
pub fn DemoLLM() -> Element {
    rsx!(
        h1 { "Demo LLM" }
        h3 { "Refereces:" }
        ul {
            li {
                Link {
                    to: NavigationTarget::<
                        Route,
                    >::External(
                        String::from("https://github.com/ealmloff/dioxus-streaming-llm/tree/main"),
                    ),
                    "ealmloff/dioxus-streaming-llm"
                }
            }
            li {
                Link {
                    to: NavigationTarget::<
                        Route,
                    >::External(String::from("https://github.com/fairjm/dioxus-openai-qa-gui")),
                    "fairjm/dioxus-openai-qa-gui"
                }
            }
            RenderLLMForm {}
        }
    )
}

#[component]
fn RenderLLMForm() -> Element {
    let _configuration = use_context_provider(|| Signal::new(Configuration::new()));
    let _system_prompts: Signal<Vec<SystemPrompt>> = use_context_provider(|| Signal::new(vec![]));
    let _system_prompt: Signal<String> = use_context_provider(|| Signal::new("".to_string()));
    let _system_prompt_dropdown: Signal<&str> = use_context_provider(|| Signal::new(""));
    let _setting_hide = use_context_provider(|| Signal::new("is-hidden"));
    let _prompt = use_context_provider(|| Signal::new(""));
    let _loading = use_context_provider(|| Signal::new(""));
    let _error_msg = use_context_provider(|| Signal::new(""));
    let _response = use_context_provider(|| {
        Signal::new(ChatResponse {
            content: String::from(""),
            prompt_tokens: 0,
            completion_tokens: 0,
        })
    });

    rsx! {
        MyCard {

            div { class: "container is-max-desktop px-2",
                RenderNav {}
                RenderSetting {}
                RenderPrompt {}
                RenderSavePrompt {}
                RenderSubmit {}
            }
        }
    }
}

fn request_button_disable(config: &Configuration, system_prompt: &str, user_prompt: &str) -> bool {
    config.secret.is_empty()
        || config.url_prefix.is_empty()
        || (system_prompt.is_empty() && user_prompt.is_empty())
}

#[component]
fn RenderSubmit() -> Element {
    let mut error_msg = use_context::<Signal<String>>();
    let mut loading = use_context::<Signal<String>>();
    let mut response = use_context::<Signal<ChatResponse>>();
    let configuration = use_context::<Signal<Configuration>>();
    let system_prompt = use_context::<Signal<String>>();
    let prompt = use_context::<Signal<String>>();

    let submit = move |_| {
        spawn(async move {
            loading.set("is-loading".to_string());
            let configuration = configuration.clone();
            let system_prompt = system_prompt.clone();
            let prompt = prompt.clone();

            let result = request(
                configuration().url_prefix.clone(),
                configuration().secret.clone(),
                system_prompt().to_string(),
                prompt().to_string(),
            )
            .await;

            match result {
                Ok(res) => {
                    error_msg.set("".to_string());
                    response.set(res);
                }
                Err(e) => error_msg.set(e.to_string()),
            }
            loading.set("".to_string());
        });
    };

    rsx!(
        button {
            class: "button is-primary my-1 {loading()}",
            disabled: "{request_button_disable(&configuration(), &system_prompt(), &prompt())}",
            onclick: submit,
            "submit"
        }

        if request_button_disable(&configuration(), &system_prompt(), &prompt()) {
            div { class: "notification is-warning",
                "please check url, openAI secret, system prompt and user prompt"
            }
        }

        if !error_msg().is_empty() {
            div { class: "notification is-warning",
                button {
                    class: "delete",
                    onclick: move |_| {
                        error_msg.set("".to_string());
                    }
                }
                "{error_msg}"
            }
        }
        if !response().content.is_empty() {
            article { class: "message mt-2",
                div {
                    class: "message-body",
                    dangerous_inner_html: "{response().content}"
                }
            }
        }
    )
}

fn save_button_attr(system_prompt: String) -> String {
    if system_prompt.trim().is_empty() {
        "is-hidden".to_string()
    } else {
        "".to_string()
    }
}

#[component]
fn RenderSavePrompt() -> Element {
    let mut system_prompt = use_context::<Signal<String>>();
    let mut system_prompt_name = use_context::<Signal<String>>();
    let mut prompt = use_context::<Signal<String>>();
    let mut system_prompts = use_context::<Signal<Vec<SystemPrompt>>>();
    let loading = use_context::<Signal<String>>();
    let save_button_hidden = save_button_attr(system_prompt());

    rsx!(
        div { class: "columns",
            div { class: "column pt-1",
                p { class: "control",
                    textarea {
                        class: "textarea",
                        value: "{system_prompt()}",
                        oninput: move |evt| {
                            system_prompt.set(evt.value().clone());
                        }
                    }
                }
                div { class: "level {save_button_hidden} mt-1",
                    div { class: "level-left",
                        div { class: "level-item",
                            input {
                                class: "input",
                                placeholder: "prompt(overwrite existing)",
                                r#type: "text",
                                value: "{system_prompt_name()}",
                                oninput: move |evt| { system_prompt_name.set(evt.value().clone()) }
                            }
                        }
                        div { class: "level-item",
                            button {
                                class: "button is-primary",
                                disabled: "{system_prompt_name().is_empty()}",
                                onclick: move |_| {
                                    system_prompts
                                        .with_mut(|e| {
                                            if let Some(v) = e.iter_mut().find(|p| p.name.eq(&system_prompt_name()))
                                            {
                                                v.content = system_prompt().clone().to_string();
                                            } else {
                                                e.push(SystemPrompt {
                                                    name: system_prompt_name().clone().to_string(),
                                                    content: system_prompt().clone().to_string(),
                                                });
                                            }
                                        });
                                    save_system_prompts_v2(system_prompts());
                                },
                                "save prompt"
                            }
                        }
                    }
                }
            }
            div { class: "column pt-1",
                p { class: "control {loading()}",
                    textarea {
                        class: "textarea",
                        value: "{prompt()}",
                        oninput: move |evt| {
                            prompt.set(evt.value().clone());
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn RenderPrompt() -> Element {
    use dioxus_free_icons::icons::bs_icons::BsArrowDownShort;
    use dioxus_free_icons::icons::bs_icons::BsArrowUpShort;

    let system_prompts = use_context::<Signal<Vec<SystemPrompt>>>();
    let mut system_prompt_name = use_context::<Signal<String>>();
    let mut system_prompt = use_context::<Signal<String>>();
    let mut system_prompt_dropdown = use_context::<Signal<&str>>();

    let system_prompts_elements = system_prompts().into_iter().map(|e| {
        let e_clone = e.clone();

        rsx!(
            div { class: "column",
                span {
                    class: "tag is-primary is-light",
                    onclick: move |_| {
                        system_prompt_name.set(e.clone().name.clone());
                        system_prompt.set(e.clone().content.clone());
                        system_prompt_dropdown.set("");
                    },
                    "{e.name}"

                    button {
                        class: "delete is-small",
                        onclick: move |_| {
                            let system_prompts_binding = system_prompts();
                            let filtered_system_prompts = system_prompts_binding
                                .iter()
                                .filter(|v| { v.name != e_clone.name })
                                .collect();
                            save_system_prompts(filtered_system_prompts);
                        }
                    }
                }
            }
        )
    });

    rsx!(
        div { class: "columns",
            div { class: "column pb-1",
                nav { class: "level mb-1",
                    div { class: "level-left",
                        div { class: "level-item",
                            p { class: "has-text-grey-light", "系统prompt" }
                        }
                        div { class: "level-item",
                            div { class: "dropdown {system_prompt_dropdown}",
                                div { class: "dropdown-trigger",
                                    button {
                                        class: "button is-small",
                                        "aria-haspopup": true,
                                        "aria-controls": "dropdown-menu",
                                        onclick: move |_| {
                                            if system_prompt_dropdown().is_empty() {
                                                system_prompt_dropdown.set("is-active");
                                            } else {
                                                system_prompt_dropdown.set("");
                                            }
                                        },
                                        span { "prompt列表" }
                                        span { class: "icon is-small",
                                            if system_prompt_dropdown().is_empty() {
                                                Icon { width: 24, height: 24, fill: "#6e7781", icon: BsArrowDownShort }
                                            } else {
                                                Icon { width: 24, height: 24, fill: "#6e7781", icon: BsArrowUpShort }
                                            }
                                        }
                                    }
                                }

                                div {

                                    class: "dropdown-menu",

                                    id: "dropdown-menu",

                                    role: "menu",
                                    div { class: "dropdown-content",
                                        a {
                                            class: "dropdown-item py-0",
                                            onclick: move |_| {
                                                system_prompt_dropdown.set("");
                                            },
                                            "关闭"
                                        }
                                        hr { class: "dropdown-divider" }
                                        if system_prompts.is_empty() {
                                            div { class: "dropdown-item",
                                                p { "没有system prompts" }
                                            }
                                        }
                                        div { class: "dropdown-item",
                                            div { class: "columns is-multiline",
                                                {system_prompts_elements}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "column pb-1",
                p { class: "has-text-grey-light", "用户prompt" }
            }
        }
    )
}

#[component]
fn RenderSetting() -> Element {
    use dioxus_free_icons::icons::bs_icons::BsGear;

    let mut setting_hide = use_context::<Signal<&str>>();
    let mut configuration = use_context::<Signal<Configuration>>();

    rsx!(
        button {
            class: "button is-white is-small",
            onclick: move |_| {
                if setting_hide().is_empty() {
                    setting_hide.set("is-hidden");
                } else {
                    setting_hide.set("");
                }
            },
            span { class: "icon has-text-light",
                Icon { width: 24, height: 24, fill: "#6e7781", icon: BsGear }
            }
            span { "setting" }
        }

        div { class: "columns {setting_hide}",
            div { class: "column is-6",
                input {
                    class: "input",
                    r#type: "text",
                    value: "{configuration().url_prefix}",
                    oninput: move |evt| {
                        let conf = Configuration {
                            url_prefix: evt.value().clone(),
                            secret: configuration().secret.clone(),
                        };
                        save_configuration(&conf);
                        configuration.set(conf);
                    }
                }
            }
            div { class: "column is-6",
                input {
                    class: "input",
                    placeholder: "OpenAi Secret",
                    r#type: "password",
                    value: "{configuration().secret}",
                    oninput: move |evt| {
                        let conf = Configuration {
                            url_prefix: configuration().url_prefix.clone(),
                            secret: evt.value().clone(),
                        };
                        save_configuration(&conf);
                        configuration.set(conf);
                    }
                }
            }
        }
    )
}

#[allow(unused)]
fn save_configuration(config: &Configuration) {
    todo!("implement save_configuration")
}

#[component]
fn RenderNav() -> Element {
    rsx!(
        nav { class: "level mt-2 mb-2",
            div { class: "level-left",
                div { class: "level-item",
                    p { class: "title is-size-4 has-text-centered", "OpenAI测试" }
                }
                div { class: "level-item",
                    a {
                        class: "button is-small",
                        target: "_blank",
                        href: "https://github.com/fairjm/dioxus-openai-qa-gui",
                        span { class: "icon is-small",
                            Icon { width: 24, height: 24, fill: "#6e7781", icon: FaGithub }
                        }
                        span { "GitHub" }
                    }
                }
            }
        }
    )
}

#[component]
fn SystemPrompt() -> Element {
    rsx!(
        h3 { "system prompt list" }
        div { class: "select",
            select {
                option { "option 01" }
                option { "option 02" }
            }
        }
        div { class: "control",
            textarea {
                class: "textarea",
                r#"type"#: "text",
                readonly: true,
                placeholder: "select a prompt"
            }
        }
        div { class: "control",
            form {
                onsubmit: move |event| {
                    info!("Submitted! {event:?}");
                },
                input { name: "name" }
                button { class: "button is-primary", "Submit" }
            }
        }
    )
}

#[component]
fn DropdownMenu() -> Element {
    let mut system_prompt_dropdown = use_context::<Signal<SystemPrompts>>();
    let system_prompts = use_context::<Signal<SystemPrompts>>();
    rsx!(
        div { class: "dropdown-menu", id: "dropdown-menu", role: "menu",
            div { class: "dropdown-content",
                a {
                    class: "dropdown-item py-0",
                    onclick: move |_| {
                        system_prompt_dropdown.set(SystemPrompts::new());
                    },
                    "关闭"
                }
                hr { class: "dropdown-divider" }
                if system_prompts().is_empty() {
                    div { class: "dropdown-item",
                        p { "没有system prompts" }
                    }
                }
            }
        }
    )
}

#[component]
fn DropdownItem() -> Element {
    // Need optimize this part
    let system_prompts = use_context::<Signal<Vec<SystemPrompt>>>();
    let mut system_prompt = use_context::<Signal<String>>();
    let mut system_prompt_name = use_context::<Signal<String>>();
    let mut system_prompt_dropdown = use_context::<Signal<String>>();

    let prompt_rendered = system_prompts().into_iter().map(|each_prompt| {
        let each_prompt_clone = each_prompt.clone();
        let system_prompts_read = system_prompts();

        rsx! {
            div { class: "column",
                span {
                    class: "tag is-primary is-light",
                    onclick: move |_| {
                        system_prompt_name.set(each_prompt_clone.name.clone());
                        system_prompt.set(each_prompt_clone.content.clone());
                        system_prompt_dropdown.set("".to_string());
                    },
                    "{each_prompt.name}"

                    button {
                        class: "delete is-small",
                        onclick: move |_| {
                            let system_prompts_read_filtered = system_prompts_read
                                .iter()
                                .filter(|each| each.name != each_prompt.name)
                                .collect::<Vec<&SystemPrompt>>();
                            save_system_prompts(system_prompts_read_filtered)
                        }
                    }
                }
            }
        }
    });

    rsx!(
        div { class: "dropdown-item",
            div { class: "columns is-multiline", {prompt_rendered} }
        }
    )
}

#[allow(unused)]
fn save_system_prompts(prompts: Vec<&SystemPrompt>) {
    todo!()
    // write_data(SYSTEM_PROMPTS_FILE_NAME, prompts);
}

#[allow(unused)]
fn save_system_prompts_v2(prompts: Vec<SystemPrompt>) {
    todo!()
    // write_data(SYSTEM_PROMPTS_FILE_NAME, prompts);
}

#[allow(unused)]
async fn request(
    url_prefix: String,
    secret: String,
    system_prompt: String,
    prompt: String,
) -> Result<ChatResponse, Box<dyn std::error::Error>> {
    let mut messages = Vec::new();
    if !system_prompt.trim().is_empty() {
        messages.push(MessageBody {
            role: String::from("ROLE_SYSTEM"),
            content: system_prompt.clone(),
        })
    }
    messages.push(MessageBody {
        role: String::from("ROLE_USER"),
        content: prompt.clone(),
    });

    let client = reqwest::Client::new();
    let body = json!({
        "messages":  messages,
        "model": "gpt-3.5-turbo",
    });

    println!("body:{}", body);

    let mut authorization = "Bearer ".to_string();
    authorization.push_str(&secret);

    let res = client
        .post(format!("{url_prefix}/v1/chat/completions"))
        .body(body.to_string())
        .header("Content-Type", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?
        .text()
        .await?;
    println!("result:{}", res);
    let v: Value = serde_json::from_str(&res)?;
    let error = v["error"]["message"].as_str();
    if let Some(e) = error {
        return Err(e.to_string().into());
    }
    let content = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_else(|| "")
        .to_string();
    let prompt_tokens = v["usage"]["prompt_tokens"].as_u64().unwrap_or_else(|| 0);
    let completion_tokens = v["usage"]["completion_tokens"]
        .as_u64()
        .unwrap_or_else(|| 0);

    // let mut path = PathBuf::new();
    // let mut file_name = current_date_time();
    // file_name.push_str(".txt");
    // path.push(OUTPUT_DIR);
    // path.push(file_name);
    // write_plain_data(path.as_path(),&format!(
    //     "system prompt:{}\nuser prompt:{}\n\nanswer:{}\nprompt_tokens:{} completion_tokens:{}\n\nfull body:{}",
    //     system_prompt, prompt, content, prompt_tokens, completion_tokens, res
    // ));
    // Ok(ChatResponse {
    //     content: markdown::to_html(&content),
    //     prompt_tokens,
    //     completion_tokens,
    // })
    todo!("implement request")
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct Configuration {
    url_prefix: String,
    secret: String,
}
#[allow(unused)]
impl Configuration {
    fn new() -> Self {
        Configuration {
            url_prefix: "".to_string(),
            secret: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct SystemPrompt {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct SystemPrompts {
    pub prompt_list: Vec<SystemPrompt>,
}

#[allow(unused)]
impl SystemPrompts {
    fn new() -> Self {
        SystemPrompts {
            prompt_list: vec![],
        }
    }
    fn is_empty(self) -> bool {
        self.prompt_list.is_empty()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct MessageBody {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct ChatResponse {
    content: String,
    prompt_tokens: u64,
    completion_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct SystemPromptDropdown {
    pub dropdown_list: Vec<String>,
}

#[allow(unused)]
impl SystemPromptDropdown {
    fn new() -> Self {
        SystemPromptDropdown {
            dropdown_list: vec![],
        }
    }
}
