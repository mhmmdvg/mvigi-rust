use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProjectListsProps {
    pub projects: Vec<&'static str>,
}

#[component]
pub fn ProjectLists(props: ProjectListsProps) -> Element {
    rsx! {
        div {
            class: "flex w-full flex-row items-end gap-6 pb-6 px-6 overflow-x-auto",
            for it in props.projects.iter() {
                div {
                    class: "h-78 aspect-video bg-gray-100 border border-gray-200 shrink-0",
                }
            }
        }
    }
}
