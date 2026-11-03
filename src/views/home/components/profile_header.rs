use dioxus::prelude::*;
use crate::components::avatar::{Avatar, AvatarSize};

#[derive(Props, Clone, PartialEq, Eq, Debug)]
pub struct ProfileHeaderProps {
    pub name: String,
    pub title: String,
}

#[component]
pub fn ProfileHeader(props: ProfileHeaderProps) -> Element {
    rsx! {
        div {
            class: "flex gap-2 items-center",
            Avatar {
                size: AvatarSize::Large,
                src: "https://avatars.githubusercontent.com/u/12345678?v=4".to_string(),
                alt: "User Avatar".to_string(),
            }

            div {
                class: "space-y-0 flex flex-col",
                h1 {
                    class: "font-medium",
                    {props.name}
                }
                p {
                    class: "text-sm text-gray-500/70",
                    {props.title}
                }
            }
        }
    }
}