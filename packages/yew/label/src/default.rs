use radix_yew_label::Label as LabelPrimitive;
use tailwind_fuse::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct LabelProps {
    // Attributes from `label`
    #[prop_or_default]
    pub r#for: Option<String>,
    #[prop_or_default]
    pub on_mouse_down: Callback<MouseEvent>,

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
pub fn Label(props: &LabelProps) -> Html {
    html! {
        <LabelPrimitive
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={tw_merge!("text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70", &props.class)}
            style={props.style.clone()}

            r#for={props.r#for.clone()}
            on_mouse_down={props.on_mouse_down.clone()}
        >
            {props.children.clone()}
        </LabelPrimitive>
    }
}
