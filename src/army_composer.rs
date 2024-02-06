#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::army::Army;

pub fn ArmyComposer(cx: Scope) -> Element {
    cx.render(rsx! {
        Header {},
        Body{}
    })
}

fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            style: "text-align: center;",
            "Header",
        }
    })
}

#[derive(PartialEq, Props)]
struct ArmyCreationPopupProps {
    is_visible: UseState<bool>,
    armies: UseRef<Vec<Army>>,
}

fn ArmyCreationPopup(cx: Scope<ArmyCreationPopupProps>) -> Element {
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
                    "Army List : "
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

fn Body(cx: Scope) -> Element {
    let armies = use_ref(cx, || Vec::<Army>::new());
    let army_creation_popup = use_state(cx, || false);
    cx.render(rsx!(
        div {
            div {
                width: "33%",
                display: "inline-block",
                "Column1",
                for army in &*armies.read() {
                    div {
                        "Army : ",
                        army.name.clone()
                    }
                }
                button {
                    class: "button",
                    onclick: move |_| {
                        army_creation_popup.set(true);
                    },
                    "New Army"
                }
            },
            div {
                width: "33%",
                display: "inline-block",
                "Column2",
            },
            div {
                width: "33%",
                display: "inline-block",
                "Column3",
            },
            if *army_creation_popup.get() {
                cx.render(rsx!(
                    ArmyCreationPopup {
                        is_visible: army_creation_popup.clone(),
                        armies: armies.clone(),
                    }
                ))
            }
            else
            {
                None
            }
        }
    ))
}
