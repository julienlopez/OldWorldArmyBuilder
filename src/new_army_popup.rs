#![allow(non_snake_case)]

use dioxus::prelude::*;

use std::collections::HashMap;

use crate::army::Army;

fn get_value_as_string(
    dict: &HashMap<std::string::String, std::vec::Vec<std::string::String>>,
    key: &str,
) -> Option<String> {
    let values = dict.get(key)?;
    if values.len() != 1 {
        return None;
    }
    Some(values[0].clone())
}

fn check_not_empty(str: String) -> Option<String> {
    if str.len() == 0 {
        return None;
    }
    Some(str)
}

fn create_army_from_form(event: FormEvent) -> Option<Army> {
    Some(Army {
        name: check_not_empty(get_value_as_string(&event.data.values, "army_name")?)?,
        points: get_value_as_string(&event.data.values, "points")?
            .parse()
            .ok()?,
        faction: get_value_as_string(&event.data.values, "faction")?,
    })
}

#[derive(PartialEq, Props)]
pub struct NewArmyPopupProps {
    is_visible: UseState<bool>,
    armies: UseRef<Vec<Army>>,
}

pub fn NewArmyPopup(cx: Scope<NewArmyPopupProps>) -> Element {
    return cx.render(rsx! {
        div {
            class: "popup_background",
            div {
                class: "popup_container",
                width: "60%",
                style: "text-align: center;",
                span {
                    class : "close",
                    onclick: |_| {cx.props.is_visible.set(false);},
                    "X"
                }
                form {
                    onsubmit: |event| {
                        if let Some(army) = create_army_from_form(event) {
                            cx.props.armies.write().push(army);
                            println!("Army Created!");
                            cx.props.is_visible.set(false);
                        }

                    },
                    "Army Name : "
                    input {
                        name: "army_name",
                    }
                    br {}
                    "Faction : "
                    select {
                        name: "faction",
                        option {
                            value: "khemri",
                            "Tomb Kings of Khemri"
                        }
                    }
                    br {}
                    "Points : "
                    input {
                        name: "points",
                    }
                    button {
                        class: "button",
                        margin: "auto",
                        "Ok"
                    },
                }
            }
        }
    });
}
