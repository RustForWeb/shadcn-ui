use radix_yew_label::Label as LabelPrimitive;
use tailwind_fuse::*;
use yew::prelude::*;
use yew_style::Style;

#[derive(PartialEq, Properties)]
pub struct LabelProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `label`
    #[prop_or_default]
    pub r#for: Option<String>,

    // Event handler attributes
    #[prop_or_default]
    pub on_mouse_down: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Label(props: &LabelProps) -> Html {
    html! {
        <LabelPrimitive
            class={tw_merge!("text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}

            r#for={props.r#for.clone()}

            on_mouse_down={props.on_mouse_down.clone()}

            node_ref={props.node_ref.clone()}
        >
            {props.children.clone()}
        </LabelPrimitive>
    }
}
