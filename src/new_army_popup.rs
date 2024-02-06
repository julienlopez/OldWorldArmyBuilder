#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::army::Army;

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
                    "Army Name : "
                    input {

                    }
                    br {}
                    "Faction : "
                    input {

                    }
                    br {}
                    "Points : "
                    input {

                    }
                    button {
                        class: "button",
                        margin: "auto",
                        prevent_default: "onclick",
                        onclick: |event| {
                            cx.props.armies.write().push(
                                Army{
                                    name: "First Army".to_string(),
                                    points: 1000,
                                    faction: "Tomb Kings of Khemri".to_string()
                                });
                            cx.props.is_visible.set(false);
                        },
                        "Ok"
                    }
                }
            }
        }
    });
}
