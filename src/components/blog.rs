use dioxus::prelude::*;

#[derive(Props, PartialEq, Eq)]
pub struct BlogProps {
    title: &'static str,
    content: &'static str
}

pub fn Blog(cx: Scope<BlogProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "center-3rd",
            div {
                class: "center-3rd-inner",
                h1 {
                    class: "blog-title",
                    "{cx.props.title}"
                },
                div {
                    class: "blog-content",
                    "{cx.props.content}"
                }
            }
        }
    })
}
