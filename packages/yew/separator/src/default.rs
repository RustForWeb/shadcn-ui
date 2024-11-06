pub use radix_yew_separator::Orientation;
use radix_yew_separator::Separator as SeparatorPrimitive;
use tailwind_fuse::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SeparatorProps {
    #[prop_or(Orientation::Horizontal)]
    pub orientation: Orientation,
    #[prop_or(true)]
    pub decorative: bool,

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
pub fn Separator(props: &SeparatorProps) -> Html {
    html! {
        <SeparatorPrimitive
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!(
                "shrink-0 bg-border",
                if props.orientation == Orientation::Horizontal {
                    "h-[1px] w-full"
                } else {
                    "h-full w-[1px]"
                },
                &props.class
            )}
            style={props.style.clone()}

            orientation={props.orientation}
            decorative={props.decorative}
        >
            {props.children.clone()}
        </SeparatorPrimitive>
    }
}
