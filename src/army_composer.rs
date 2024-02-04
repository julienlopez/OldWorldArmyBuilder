#![allow(non_snake_case)]

use dioxus::prelude::*;

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
    cx.render(rsx! {
        div {
            style: "columns: 3",
            div {
                "Column1",
            }
            div {
                "Column2",
            }
            div {
                "Column3",
            }
        }
    })
}
