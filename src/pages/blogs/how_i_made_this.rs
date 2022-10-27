use dioxus::prelude::*;
use crate::components;

pub fn HowIMadeThis(cx: Scope) -> Element {
    cx.render(rsx! {
        components::Blog {
            title: "How I Made This Website",
            content: "With Rust 🦀🦀🦀"
        }
    })
}
