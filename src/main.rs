#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

pub mod army;
pub mod army_composer;

use army_composer::ArmyComposer;

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(WindowBuilder::new().with_resizable(true).with_inner_size(
            dioxus_desktop::wry::application::dpi::LogicalSize::new(800.0, 600.0),
        )),
    );
}

fn App(cx: Scope) -> Element {
    let popup_visible = use_state(cx, || true);
    dbg!(popup_visible);
    cx.render(rsx! {
        main {
            style { include_str!("./style.css") }
            ArmyComposer {}
        }
    })
}
