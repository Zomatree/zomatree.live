#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_web::launch;
use dioxus_router::{Route, Router};

mod components;
mod pages;

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route {
                to: "/",
                pages::App {}
            },
            Route {
                to: "/blogs/how_i_made_this",
                pages::blogs::HowIMadeThis {}
            }
        }
    })
}

fn main() {
    launch(app)
}
