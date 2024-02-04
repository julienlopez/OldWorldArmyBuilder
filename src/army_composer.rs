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

fn Body(cx: Scope) -> Element {
    let armies = use_state(cx, || Vec::<Army>::new());
    render! {
        div {
            div {
                width: "33%",
                display: "inline-block",
                "Column1"
                // for army in &armies {
                //     div {
                //         "Army : ",
                //         &army.name
                //     }
                // }
                button {
                    class: "button",
                    "New Army"
                }
            }
            div {
                width: "33%",
                display: "inline-block",
                "Column2",
            }
            div {
                width: "33%",
                display: "inline-block",
                "Column3",
            }
        }
    }
}
