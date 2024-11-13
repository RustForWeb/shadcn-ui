use tailwind_fuse::*;
use yew::prelude::*;

#[derive(TwClass)]
#[tw(
    class = "relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground"
)]
pub struct AlertClass {
    pub variant: AlertVariant,
}

#[derive(PartialEq, TwVariant)]
pub enum AlertVariant {
    #[tw(default, class = "bg-background text-foreground")]
    Default,
    #[tw(
        class = "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive"
    )]
    Destructive,
}

#[derive(PartialEq, Properties)]
pub struct AlertProps {
    #[prop_or_default]
    pub variant: AlertVariant,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Alert(props: &AlertProps) -> Html {
    let class = use_memo((props.variant, props.class.clone()), |(variant, class)| {
        AlertClass { variant: *variant }.with_class(class.clone().unwrap_or_default())
    });

    html! {
        <div
            ref={props.node_ref.clone()}

            class={(*class).clone()}
            id={props.id.clone()}
            role="alert"
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct AlertTitleProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn AlertTitle(props: &AlertTitleProps) -> Html {
    html! {
        <h5
            ref={props.node_ref.clone()}

            class={tw_merge!("mb-1 font-medium leading-none tracking-tight", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </h5>
    }
}

#[derive(PartialEq, Properties)]
pub struct AlertDescriptionProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn AlertDescription(props: &AlertDescriptionProps) -> Html {
    html! {
        <div
            ref={props.node_ref.clone()}

            class={tw_merge!("text-sm [&_p]:leading-relaxed", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}
