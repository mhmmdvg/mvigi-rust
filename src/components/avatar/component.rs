use dioxus::prelude::*;


#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarSize {
    #[default]
    Small,
    Medium,
    Large,
}

impl AvatarSize {
    fn to_class(self) -> &'static str {
        match self {
            AvatarSize::Small => "w-8 h-8",
            AvatarSize::Medium => "w-12 h-12",
            AvatarSize::Large => "w-16 h-16",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Square,
}

impl AvatarShape {
    fn to_class(self) -> &'static str {
        match self {
            AvatarShape::Circle => "rounded-full",
            AvatarShape::Square => "rounded-none",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /* Image URL. Use `asset!()` for local files, or any remote URL. */
    pub src: String,

    #[props(default)]
    pub alt: String,

    /* Callback when image loads successfully. */
    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    /* Callback when image fails to load. */
    #[props(default)]
    pub on_error: Option<EventHandler<()>>,

    #[props(default)]
    pub size: AvatarSize,

    #[props(default)]
    pub shape: AvatarShape,

    /* Additional attributes for the avatar element. */
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /* Fallback (e.g. initials) shown while loading and if the image fails. */
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    // Remember which src failed, so a new src gets a fresh attempt.
    let mut failed_src = use_signal(|| None::<String>);
    let failed = failed_src.read().as_deref() == Some(props.src.as_str());
    let src = props.src.clone();

    rsx! {
        div {
            class: "relative inline-flex shrink-0 items-center justify-center overflow-hidden bg-gray-200 text-gray-600 {props.size.to_class()} {props.shape.to_class()}",
            ..props.attributes,
            // Fallback sits underneath; the image covers it once loaded.
            {props.children}
            if !failed {
                img {
                    class: "absolute inset-0 h-full w-full object-cover",
                    src: props.src.clone(),
                    alt: props.alt.clone(),
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
                    },
                }
            }
        }
    }
}
