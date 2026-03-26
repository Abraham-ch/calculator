use dioxus::{prelude::*};
use tracing::Level;


const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    let values = ["1","2","3","4","5","6","7","8","9","0"];
    let mut input_value: Signal<String> = use_signal(|| String::from("0"));
    let mut operation_value: Signal<String> = use_signal(|| String::new());
    let operations: [&str; 4] = ["+", "-", "x", "="];
    let mut waiting_second_value: Signal<bool> = use_signal(|| false);
    let mut first_value: Signal<String> = use_signal(|| String::new());
    let mut second_value: Signal<String> = use_signal(|| String::new());

    let params = &Params {
        first_param:first_value.read().to_string(), 
        second_param:second_value.read().to_string(), 
        operation:operation_value.read().to_string()
    };
    let result = procedure(params);

    rsx! {
        main { class: "text-xl font-bold",
            section { class: "grid grid-cols-4 min-h-dvh min-w-full",
                input {
                    class: "border col-span-4 text-right pr-4",
                    placeholder: "0",
                    value: input_value,
                }
                for (i , value) in values.into_iter().enumerate() {
                    if value == "0" {
                        button {
                            key: "{value} pm",
                            onclick: move |_e| input_value.push_str(value),
                            class: "border cursor-pointer",
                            "+/-"
                        }
                    }
                    button {
                        onclick: move |_e| {
                            if input_value() == "0" {
                                input_value.set(value.to_string());
                                first_value.set(value.to_string());
                            } else {
                                input_value.push_str(value);
                                if *waiting_second_value.read() {
                                    second_value.push_str(value);
                                } else {
                                    first_value.push_str(value);
                                }
                            }
                        },
                        class: "border cursor-pointer",
                        "{value}"
                    }
                    if value == "0" {
                        button {
                            key: "{i} dot",
                            onclick: move |_e| input_value.push_str(value),
                            class: "border cursor-pointer",
                            "."
                        }
                    }
                }
                div { class: "col-span-1 row-span-4 col-start-4 grid grid-cols-1 row-start-2",
                    for operation in operations {
                        button {
                            class: "border",
                            onclick: move |_e| {
                                waiting_second_value.set(true);
                                operation_value.set(operation.to_string());
                                input_value.push_str(operation);

                                if operation == "=" {
                                    input_value.set(result.to_string());
                                    first_value.set(result.to_string());
                                    second_value.set(String::new());
                                }
                            },
                            "{operation}"
                        }
                    }
                }
            }
            section { class: "mt-4",
                ul {
                    li { "Your first value is: {first_value}" }
                    li { "Your operation is: {operation_value}" }
                    li { "Your second value is: {second_value}" }
                }
            }
        }
    }
}

fn parse_param(param: String) -> f64{
    match param.parse() {
        Ok(num) => num,
        Err(_e) => {
            eprintln!("Failed to parse {param}");
            0.0
        }
    }
}

fn procedure(params: &Params) -> f64 {
    
    let p1:f64 = parse_param(params.first_param.clone());
    let p2:f64 = parse_param(params.second_param.clone());

    match params.operation.as_str() {
        "+" => p1 + p2,
        "-" => p1 - p2,
        "*" => p1 * p2,
        "/" => p1 / p2,
        _   => 0.0
    }
}

#[derive(Clone, PartialEq)]
struct Params {
    first_param: String,
    second_param: String,
    operation: String
}