use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ImageAsyncProps {
    pub src: String,

    #[props(into, optional)]
    class: Option<String>,

    #[props(default)]
    pub alt: String,

    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    #[props(default)]
    pub on_error: Option<EventHandler<()>>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

}


#[component]
pub fn ImageAsync(props: ImageAsyncProps) -> Element {
    let mut failed_src = use_signal(|| None::<String>);
    let failed = failed_src.read().as_deref() == Some(props.src.as_str());
    let src = props.src.clone();

    rsx! {
        if !failed {
            img {
                class: props.class.unwrap_or_else(|| "w-6 h-6".into()),
                src: props.src.clone(),
                alt: props.alt,
                loading: "lazy",
                decoding: "async",
                onload: move |_| {
                    if let Some(cb) = props.on_load {
                        cb.call(());
                    }
                },
                onerror: move |_| {
                    failed_src.set(Some(src.clone()));
                    if let Some(cb) = props.on_error {
                        cb.call(());
                    }
                }
            }
        }
    }
}