#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::army::Army;
use crate::new_army_popup::NewArmyPopup;

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
                    NewArmyPopup {
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
