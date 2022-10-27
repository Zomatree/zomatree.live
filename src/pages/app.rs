use crate::components;
use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "intro",
            div {
                class: "intro-inner",
                div {
                    class: "name",
                    "Hi.",
                    div {
                        class: "emphasis",
                        "I'm Zomatree."
                    }
                },
                a {
                    class: "github-link",
                    href: "https://github.com/Zomatree",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "github.com/Zomatree »"
                },
            },
            div {
                class: "flex-center",
                span {
                    class: "rotate-down emphasis",
                    "》"
                }
            }
        },
        div {
            class: "flex-center",
            h1 {
                class: "emphasis",
                "Blog Posts"
            }
        },
        div {
            class: "blog-post-grid",
            components::BlogPost {
                name: "How i made this website",
                image: "how_i_made_this_website.png",
                route: "how_i_made_this"
            }
        }
    })
}
