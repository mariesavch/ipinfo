#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::Deserialize;
use sir::{css, global_css, AppStyle};

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

    global_css!(
        "
       :root {
        --rosewater: #ff8389;
        --flamingo: #ff8389;
        --red: #ff8389;
        --maroon: #ff8389;
        --pink: #ff7eb6;
        --mauve: #be95ff;
        --peach: #d44a1c;
        --yellow: #ab8600;
        --green: #08bdba;
        --teal: #33b1ff;
        --sky: #33b1ff;
        --sapphire: #33b1ff;
        --blue: #78a9ff;
        --lavender: #78a9ff;
        --text: #ffffff;
        --subtext1: #f4f4f4;
        --subtext0: #e0e0e0;
        --overlay2: #adadad;
        --overlay1: #949494;
        --overlay0: #7a7a7a;
        --surface2: #4f4f4f;
        --surface1: #383838;
        --surface0: #2e2e2e;
        --base: #161616;
        --mantle: #0d0d0d;
        --crust: #000000;
    } 

    @media (prefers-color-scheme: light) {
        :root {
            --rosewater: #da1e28;
            --flamingo: #da1e28;
            --red: #da1e28;
            --maroon: #da1e28;
            --pink: #d02670;
            --mauve: #8a3ffc;
            --peach: #d44a1c;
            --yellow: #ab8600;
            --green: #007d79;
            --teal: #1192e8;
            --sky: #1192e8;
            --sapphire: #1192e8;
            --blue: #0f62fe;
            --lavender: #0f62fe;
            --text: #000000;
            --subtext1: #404040;
            --subtext0: #474747;
            --overlay2: #575757;
            --overlay1: #595959;
            --overlay0: #737373;
            --surface2: #8c8c8c;
            --surface1: #d1d1d1;
            --surface0: #e6e6e6;
            --base: #ffffff;
            --mantle: #f2f2f2;
            --crust: #ebebeb;
        }
    }

    :root {
        background-color: var(--base);
        color: var(--text);
        line-height: 1.6;
    }

    "
    );

    let animated_list = css!(
        "
    @media (hover: hover) and (pointer: fine) {
        li {
            transition-property: all;
            transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
            transition-duration: 300ms;
        }
        &:hover li {
            opacity: 0.5;
        }
        &:hover li:hover {
            opacity: 1;
        }
    }
    "
    );

    let item = css!(
        "
        display: flex;
        flex-direction: column;
        padding-top: 12px;
        padding-bottom: 12px;
        gap: 4px;"
    );

    let input = css!(
        "
        all: unset;
        padding-top: 0.5rem;
        padding-bottom: 0.5rem; 
        padding-left: 1rem;
        padding-right: 1rem;
        border-radius: 0.375rem; 
        border: 1px solid var(--surface0); 
        text-transform: capitalize; 
        transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 300ms; 
        color: var(--text);

        &:hover {
            border-color: var(--surface1);
        }
        &:focus {
            border-color: var(--surface2);
        }
        &:placeholder {
            color: var(--overlay0);
        }
        "
    );

    let section = css!(
        "
    padding-top: 24px;
    @media(min-width: 950px) {
        padding-top: 64px
    }
    "
    );

    rsx! {
        AppStyle {}
        main {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            div { class: section,
                input {
                    aria_label: "Enter ip",
                    placeholder: "Enter ip",
                    spellcheck: false,
                    value: ip,
                    r#type: "text",
                    autofocus: true,
                    oninput: move |event| ip.set(event.value()),
                    class: input,
                }
                div { margin_top: "24px",
                    if let Some(Ok(data)) = ipinfo.read().as_ref() {
                        ul {
                            class: animated_list,
                            all: "unset",
                            list_style_type: "none",
                            display: "grid",
                            grid_template_columns: "repeat(auto-fit, minmax(300px, 1fr))",
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "IP" }
                                    span { "{data.ip}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Hostname" }
                                    span { "{data.hostname}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "City" }
                                    span { "{data.city}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Region" }
                                    span { "{data.region}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Country" }
                                    span { "{data.country}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Location" }
                                    span { "{data.loc}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Org" }
                                    span { "{data.org}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Timezone" }
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
