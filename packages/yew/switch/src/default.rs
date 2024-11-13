use radix_yew_switch::{Switch as SwitchPrimitive, SwitchThumb as SwitchThumbPrimitive};
use tailwind_fuse::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SwitchProps {
    // Props from `SwitchPrimitive`
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub default_checked: Option<bool>,
    #[prop_or_default]
    pub on_checked_change: Callback<bool>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    // Attributes from `button`
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or("on".to_owned())]
    pub value: String,

    // Event handler attributes
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
}

#[function_component]
pub fn Switch(props: &SwitchProps) -> Html {
    html! {
        <SwitchPrimitive
            checked={props.checked}
            default_checked={props.default_checked}
            on_checked_change={props.on_checked_change.clone()}

            class={tw_merge!(
                "peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input",
                &props.class
            )}
            id={props.id.clone()}
            style={props.style.clone()}

            disabled={props.disabled}
            name={props.name.clone()}
            required={props.required}
            value={props.value.clone()}

            on_click={props.on_click.clone()}

            node_ref={props.node_ref.clone()}
        >
            <SwitchThumbPrimitive
                class={tw_merge!{
                    "pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
                }}
            />
        </SwitchPrimitive>
    }
}
