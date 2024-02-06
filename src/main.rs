#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

pub mod army;
pub mod army_composer;
mod new_army_popup;

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
    cx.render(rsx! {
        main {
            style { include_str!("./style.css") }
            ArmyComposer {}
        }
    })
}
