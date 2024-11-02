use tailwind_fuse::*;
use yew::prelude::*;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground shadow hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90")]
    Destructive,
    #[tw(
        class = "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground"
    )]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonSize {
    #[tw(default, class = "h-9 px-4 py-2")]
    Default,
    #[tw(class = "h-8 rounded-md px-3 text-xs")]
    Sm,
    #[tw(class = "h-10 rounded-md px-8")]
    Lg,
    #[tw(class = "h-9 w-9")]
    Icon,
}

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,

    // Attributes from `button`
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<ButtonChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct ButtonChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: String,
    pub style: Option<String>,
    pub disabled: bool,
    pub onclick: Callback<MouseEvent>,
}

impl ButtonChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <button
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                disabled={self.disabled}
                onclick={self.onclick}
            >
                {children}
            </button>
        }
    }
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let class = use_memo(
        (props.variant, props.size, props.class.clone()),
        |(variant, size, class)| {
            ButtonClass {
                variant: *variant,
                size: *size,
            }
            .with_class(class.clone().unwrap_or_default())
        },
    );

    let child_props = ButtonChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: (*class).clone(),
        style: props.style.clone(),
        disabled: props.disabled,
        onclick: props.on_click.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
