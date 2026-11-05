use super::components::{ContentContainer, ProfileHeader, ProjectLists};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        main {
            class: "flex min-h-dvh flex-col w-full overflow-hidden",
            div {
                class: "flex flex-1 items-center justify-center",
                div {
                    class: "flex w-full max-w-[600px] flex-col gap-6",
                    ProfileHeader {
                        name: "Muhammad Vikri",
                        title: "Software Engineer",
                    }

                    p {
                        "I love building interfaces and software, obsessing over every detail until the experience feels right."
                    }

                    p {
                        "I run Interface Office, a design engineering studio helping AI startups build their products."
                    }

                    div {
                        class: "grid grid-cols-2",
                        div {
                            class: "flex flex-col gap-5",
                            ContentContainer {
                                title: "Recent Projects",
                                content: vec![
                                    "Glampify",
                                    "Background Synthesize",
                                    "URL Capture"
                                ]
                            }

                            ContentContainer {
                                title: "Achievments",
                                content: vec![
                                    ""
                                ]
                            }
                        }

                        div {
                            h3 {
                                class: "text-black/60",
                                "Recent Blog Posts"
                            }
                            p {
                                "Building a design system for AI startups"
                            }
                        }
                    }
                }
            }

            ProjectLists {
                projects: vec![
                    "Project 1",
                    "Project 2",
                    "Project 3",
                    "Project 4",
                    "Project 5",
                    "Project 6",
                ]
            }
        }
    }
}
