use radix_yew_avatar::{
    Avatar as AvatarPrimitive, AvatarFallback as AvatarFallbackPrimitive,
    AvatarImage as AvatarImagePrimitive, ImageLoadingStatus,
};
use tailwind_fuse::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AvatarProps {
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
pub fn Avatar(props: &AvatarProps) -> Html {
    html! {
        <AvatarPrimitive
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!("relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full", &props.class)}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </AvatarPrimitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct AvatarImageProps {
    #[prop_or_default]
    pub on_loading_status_change: Callback<ImageLoadingStatus>,

    // Attributes from `img`
    #[prop_or_default]
    pub alt: Option<String>,
    #[prop_or_default]
    pub crossorigin: Option<String>,
    #[prop_or_default]
    pub decoding: Option<String>,
    #[prop_or_default]
    pub fetchpriority: Option<String>,
    #[prop_or_default]
    pub height: Option<String>,
    #[prop_or_default]
    pub ismap: bool,
    #[prop_or_default]
    pub loading: Option<String>,
    #[prop_or_default]
    pub referrerpolicy: Option<String>,
    #[prop_or_default]
    pub sizes: Option<String>,
    #[prop_or_default]
    pub src: Option<String>,
    #[prop_or_default]
    pub srcset: Option<String>,
    #[prop_or_default]
    pub width: Option<String>,
    #[prop_or_default]
    pub usemap: Option<String>,

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
pub fn AvatarImage(props: &AvatarImageProps) -> Html {
    html! {
        <AvatarImagePrimitive
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!("aspect-square h-full w-full", &props.class)}
            style={props.style.clone()}

            on_loading_status_change={props.on_loading_status_change.clone()}

            alt={props.alt.clone()}
            crossorigin={props.crossorigin.clone()}
            decoding={props.decoding.clone()}
            fetchpriority={props.fetchpriority.clone()}
            height={props.height.clone()}
            ismap={props.ismap}
            loading={props.loading.clone()}
            referrerpolicy={props.referrerpolicy.clone()}
            sizes={props.sizes.clone()}
            src={props.src.clone()}
            srcset={props.srcset.clone()}
            width={props.width.clone()}
            usemap={props.usemap.clone()}
        />
    }
}

#[derive(PartialEq, Properties)]
pub struct AvatarFallbackProps {
    #[prop_or_default]
    pub delay_ms: Option<i32>,

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
pub fn AvatarFallback(props: &AvatarFallbackProps) -> Html {
    html! {
        <AvatarFallbackPrimitive
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!("flex h-full w-full items-center justify-center rounded-full bg-muted", &props.class)}
            style={props.style.clone()}

            delay_ms={props.delay_ms}
        >
            {props.children.clone()}
        </AvatarFallbackPrimitive>
    }
}
