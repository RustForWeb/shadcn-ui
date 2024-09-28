use std::rc::Rc;

use tailwind_fuse::{tw_merge, AsTailwindClass};
use yew::{prelude::*, virtual_dom::VTag};
use yew_attrs::{attrs, Attrs};

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl AsTailwindClass for ButtonVariant {
    fn as_class(&self) -> &str {
        match self {
            ButtonVariant::Default => "bg-primary text-primary-foreground shadow hover:bg-primary/90",
            ButtonVariant::Destructive => {
                "bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90"
            }
            ButtonVariant::Outline => {
                "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground"
            }
            ButtonVariant::Secondary => {
                "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80"
            }
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

impl AsTailwindClass for ButtonSize {
    fn as_class(&self) -> &str {
        match self {
            ButtonSize::Default => "h-9 px-4 py-2",
            ButtonSize::Sm => "h-8 rounded-md px-3 text-xs",
            ButtonSize::Lg => "h-10 rounded-md px-8",
            ButtonSize::Icon => "h-9 w-9",
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: Option<ButtonVariant>,
    #[prop_or_default]
    pub size: Option<ButtonSize>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    pub children: Html,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let base_class = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

    let variant = props.variant.clone().unwrap_or(ButtonVariant::Default);
    let size = props.size.clone().unwrap_or(ButtonSize::Default);
    let class = props.class.clone().unwrap_or_default();

    let combined_class = &tw_merge!(base_class.to_owned(), variant, size, &class);

    let merged_attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
               class={combined_class}
            })
            .expect("Attributes should be merged.")
    });

    let attrs = Rc::unwrap_or_clone(merged_attrs);

    attrs
        .new_vtag(
            "button",
            props.node_ref.clone(),
            Default::default(),
            props.children.clone(),
        )
        .into()
}
