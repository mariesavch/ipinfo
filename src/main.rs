#![allow(non_snake_case)]

use css_in_rs::{make_styles, use_style_provider_quickstart, Classes, EmptyTheme};
use dioxus::prelude::*;
use serde::Deserialize;

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

make_styles! {
    (_theme: EmptyTheme) -> MyClasses {
        ":root" {
            __rosewater: "#ff8389",
            __flamingo: "#ff8389",
            __red: "#ff8389",
            __maroon: "#ff8389",
            __pink: "#ff7eb6",
            __mauve: "#be95ff",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#08bdba",
            __teal: "#33b1ff",
            __sky: "#33b1ff",
            __sapphire: "#33b1ff",
            __blue: "#78a9ff",
            __lavender: "#78a9ff",
            __text: "#ffffff",
            __subtext1: "#f4f4f4",
            __subtext0: "#e0e0e0",
            __overlay2: "#adadad",
            __overlay1: "#949494",
            __overlay0: "#7a7a7a",
            __surface2: "#4f4f4f",
            __surface1: "#383838",
            __surface0: "#2e2e2e",
            __base: "#161616",
            __mantle: "#0d0d0d",
            __crust: "#000000",
        },
    "@media (prefers-color-scheme: light)" {
        ":root" {
            __rosewater: "#da1e28",
            __flamingo: "#da1e28",
            __red: "#da1e28",
            __maroon: "#da1e28",
            __pink: "#d02670",
            __mauve: "#8a3ffc",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#007d79",
            __teal: "#1192e8",
            __sky: "#1192e8",
            __sapphire: "#1192e8",
            __blue: "#0f62fe",
            __lavender: "#0f62fe",
            __text: "#000000",
            __subtext1: "#404040",
            __subtext0: "#474747",
            __overlay2: "#575757",
            __overlay1: "#595959",
            __overlay0: "#737373",
            __surface2: "#8c8c8c",
            __surface1: "#d1d1d1",
            __surface0: "#e6e6e6",
            __base: "#ffffff",
            __mantle: "#f2f2f2",
            __crust: "#ebebeb",
            }
        },
        ":root" {
            background_color: "var(--base)",
            color: "var(--text)",
            line_height: "1.6",
            font_family: "Cartograph CF",
        },
        "@media (hover: hover) and (pointer: fine)" {
            ".animated_list li" {
                all: "unset",
                transition_property: "all",
                transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
                transition_duration: "300ms",
            },
            ".animated_list:hover li" {
                opacity: "0.5",
            },
            ".animated_list:hover li:hover" {
                opacity: "1",
            }
        },
        ".item" {
            display: "flex",
            flex_direction: "column",
            padding_top: "12px",
            padding_bottom: "12px",
            gap: "4px",
        },
        ".input" {
            all: "unset",
            padding_top: "0.5rem",
            padding_bottom: "0.5rem",
            padding_left: "1rem",
            padding_right: "1rem",
            border_radius: "0.375rem",
            border: "1px solid var(--surface0)",
            text_transform: "capitalize",
            transition_property: "color, background-color, border-color, text-decoration-color, fill, stroke",
            transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
            transition_duration: "300ms",
            color: "var(--text)",
        },
        ".input:hover" {
            border_color: "var(--surface1)",
        },
        ".input:focus" {
            border_color: "var(--surface2)",
        },
        ".input:placeholder" {
            color: "var(--overlay0)",
        },
        ".section" {
            padding_top: "24px",
        },
        "@media(min-width: 950px)" {
            ".section" {
                padding_top: "64px",
            }
        },
    }
}

#[component]
fn App() -> Element {
    use_style_provider_quickstart(|| EmptyTheme);
    let cls: &MyClasses = &MyClasses::use_style();

    let mut ip = use_signal(|| "".to_string());
    let ipinfo = use_resource(move || async move { get_ipinfo(ip.to_string()).await });

    rsx! {
        style {
            "
            @font-face {{
            font-family: 'Cartograph CF';
            src: url('./assets/fonts/CartographCF-Regular.woff2') format('woff2');
            }}

            @font-face {{
            font-family: 'Cartograph CF';
            src: url('./assets/fonts/CartographCF-Bold.woff2') format('woff2');
            font-weight: bold;
            }}"
        }
        main {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            div { class: &cls.section as &str,
                input {
                    aria_label: "Enter ip",
                    placeholder: "Enter ip",
                    spellcheck: false,
                    value: ip,
                    r#type: "text",
                    autofocus: true,
                    oninput: move |event| ip.set(event.value()),
                    class: &cls.input as &str,
                }
                div { margin_top: "24px",
                    if let Some(Ok(data)) = ipinfo.read().as_ref() {
                        ul {
                            class: &cls.animated_list as &str,
                            all: "unset",
                            list_style_type: "none",
                            display: "grid",
                            grid_template_columns: "repeat(auto-fit, minmax(300px, 1fr))",
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "IP" }
                                    span { "{data.ip}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Hostname" }
                                    span { "{data.hostname}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "City" }
                                    span { "{data.city}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Region" }
                                    span { "{data.region}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Country" }
                                    span { "{data.country}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Location" }
                                    span { "{data.loc}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Org" }
                                    span { "{data.org}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
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
