use dioxus::prelude::*;
use dioxus_router::Link;

#[derive(Props, PartialEq, Eq)]
pub struct BlogPostProps {
    name: &'static str,
    image: &'static str,
    route: &'static str,
}

pub fn BlogPost(cx: Scope<BlogPostProps>) -> Element {
    cx.render(rsx! {
        Link {
            class: "blog-post",
            to: "/blogs/{cx.props.route}",
            div {
                class: "blog-post-inner",
                img {
                    class: "blog-post-cover",
                    src: "/{cx.props.image}"
                }
                span {
                    class: "blog-post-title",
                    "{cx.props.name}"
                }
            }
        }
    })
}
