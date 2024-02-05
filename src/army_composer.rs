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
    let armies = use_ref(cx, || Vec::<Army>::new());
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
                    onclick: |_| { armies.write().push(Army{name: "First Army".to_string(), points: 1000, faction: "Tomb Kings of Khemri".to_string()})},
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
    ))
}
