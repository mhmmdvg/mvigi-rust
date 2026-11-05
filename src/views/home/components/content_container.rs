use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ContentContainerProps {
    #[props(into)]
    pub title: String,
    pub content: Vec<&'static str>,
}

#[component]
pub fn ContentContainer(props: ContentContainerProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col",
            h3 {
                class: "text-black/60",
                {props.title}
            }

            for it in props.content.iter() {
                p { "{it}" }
            }
        }
    }
}
