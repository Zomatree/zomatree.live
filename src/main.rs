use dioxus::prelude::*;
use dioxus_web::launch;

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "intro",
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
            }
        }
    })
}

fn main() {
    launch(app)
}
