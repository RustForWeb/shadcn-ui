use tailwind_fuse::*;
use yew::prelude::*;
use yew_style::Style;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
)]
pub struct BadgeClass {
    pub variant: BadgeVariant,
}

#[derive(PartialEq, TwVariant)]
pub enum BadgeVariant {
    #[tw(
        default,
        class = "border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80"
    )]
    Default,
    #[tw(
        class = "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80"
    )]
    Secondary,
    #[tw(
        class = "border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80"
    )]
    Destructive,
    #[tw(class = "text-foreground")]
    Outline,
}

#[derive(PartialEq, Properties)]
pub struct BadgeProps {
    #[prop_or_default]
    pub variant: BadgeVariant,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Badge(props: &BadgeProps) -> Html {
    let class = use_memo((props.variant, props.class.clone()), |(variant, class)| {
        BadgeClass { variant: *variant }.with_class(class.clone().unwrap_or_default())
    });

    html! {
        <div
            ref={props.node_ref.clone()}

            class={(*class).clone()}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}
