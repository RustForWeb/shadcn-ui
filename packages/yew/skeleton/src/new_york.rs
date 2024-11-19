use tailwind_fuse::*;
use yew::prelude::*;
use yew_style::Style;

#[derive(PartialEq, Properties)]
pub struct SkeletonProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
}

#[function_component]
pub fn Skeleton(props: &SkeletonProps) -> Html {
    html! {
        <div
            ref={props.node_ref.clone()}

            class={tw_merge!("animate-pulse rounded-md bg-primary/10", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}
        />
    }
}
