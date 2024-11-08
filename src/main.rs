#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::Deserialize;
use tailwind_fuse::tw_merge;

fn main() {
    dioxus::launch(App);
}

#[derive(Debug, Deserialize, Clone)]
pub struct IpinfoResponce {
    pub ip: String,
    pub hostname: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: String,
    pub postal: String,
    pub timezone: String,
}

async fn get_ipinfo(ip: String) -> reqwest::Result<IpinfoResponce> {
    reqwest::get(format!("https://ipinfo.io/{}?token=63506fa5ae6d29", ip))
        .await?
        .json::<IpinfoResponce>()
        .await
}

#[component]
fn App() -> Element {
    let mut ip = use_signal(|| "".to_string());
    let ipinfo = use_resource(move || async move { get_ipinfo(ip.to_string()).await });
    let item = "flex py-3 flex-col gap-1";

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        main { class: "mx-auto max-w-[850px] px-6 pb-20",
            div { class: "pt-6 min-[950px]:pt-16",
                input {
                    aria_label: "Enter ip",
                    placeholder: "Enter ip",
                    spellcheck: false,
                    value: ip,
                    r#type: "text",
                    autofocus: true,
                    oninput: move |event| ip.set(event.value()),
                    class: tw_merge!(
                        "rounded-md border border-surface0 bg-base py-2 px-4",
                        "outline-none transition-colors duration-300",
                        "placeholder:text-overlay0 hover:border-surface1",
                        "focus:text-text focus:border-surface2"
                    ),
                }
                div { class: "mt-6",
                    if let Some(Ok(data)) = ipinfo.read().as_ref() {
                        ul { class: "animated-list grid grid-cols-1 sm:grid-cols-2 mt-5",
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "IP" }
                                    span { "{data.ip}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Hostname" }
                                    span { "{data.hostname}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "City" }
                                    span { "{data.city}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Region" }
                                    span { "{data.region}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Country" }
                                    span { "{data.country}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Location" }
                                    span { "{data.loc}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Org" }
                                    span { "{data.org}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Timezone" }
                                    span { "{data.timezone}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
