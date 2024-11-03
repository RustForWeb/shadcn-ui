use tailwind_fuse::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    html! {
        <div
            ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!("rounded-lg border bg-card text-card-foreground shadow-sm", &props.class)}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn CardHeader(props: &CardHeaderProps) -> Html {
    html! {
        <div
            ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!("flex flex-col space-y-1.5 p-6", &props.class)}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CardTitleProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn CardTitle(props: &CardTitleProps) -> Html {
    html! {
        <div
            ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!("text-2xl font-semibold leading-none tracking-tight", &props.class)}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CardDescriptionProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn CardDescription(props: &CardDescriptionProps) -> Html {
    html! {
        <div
            ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!("text-sm text-muted-foreground", &props.class)}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CardContentProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn CardContent(props: &CardContentProps) -> Html {
    html! {
        <div
            ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!("p-6 pt-0", &props.class)}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CardFooterProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn CardFooter(props: &CardFooterProps) -> Html {
    html! {
        <div
            ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!("flex items-center p-6 pt-0", &props.class)}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}
