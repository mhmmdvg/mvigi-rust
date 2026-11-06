use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Shadow() -> Element {
    rsx! {
        div {
            class: "pointer-events-none fixed left-0 top-0 z-50 h-16 w-full bg-gradient-to-b from-white via-white/50 to-transparent backdrop-blur-md [mask-image:linear-gradient(to_bottom,black_30%,transparent)] md:h-20 lg:h-32"
        }

        Outlet::<Route> {}
    }
}
