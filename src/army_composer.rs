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
            "Header",
        }
    })
}

fn Body(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "body",
        }
    })
}
