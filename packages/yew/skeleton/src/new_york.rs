use tailwind_fuse::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SkeletonProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
}

#[function_component]
pub fn Skeleton(props: &SkeletonProps) -> Html {
    html! {
        <div
            ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!("animate-pulse rounded-md bg-primary/10", props.class.clone())}
            style={props.style.clone()}
        />
    }
}
