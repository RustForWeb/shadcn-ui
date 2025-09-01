use yew::prelude::*;
use yew_struct_component::{StructComponent, struct_component};
use yew_style::Style;
use web_sys::MouseEvent;

pub fn checkbox_class() -> &'static str {
    "peer h-4 w-4 shrink-0 rounded-sm border border-primary ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground"
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "button")]
pub struct CheckboxChildProps {
    pub node_ref: NodeRef,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,

    // Button attributes
    pub disabled: bool,
    pub r#type: String,
    pub role: String,
    pub aria_checked: String,
    pub data_state: String,

    // Event handlers
    pub onclick: Callback<MouseEvent>,
}

#[derive(PartialEq, Properties)]
pub struct CheckboxProps {
    /// Whether the checkbox is checked
    #[prop_or_default]
    pub checked: bool,
    
    /// Callback when checked state changes
    #[prop_or_default]
    pub on_checked_change: Option<Callback<bool>>,
    
    /// Whether the checkbox is disabled
    #[prop_or_default]
    pub disabled: bool,
    
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
    pub as_child: Option<Callback<CheckboxChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Checkbox(props: &CheckboxProps) -> Html {
    let checked = use_state(|| props.checked);
    
    let handle_click = {
        let checked = checked.clone();
        let on_change = props.on_checked_change.clone();
        let disabled = props.disabled;
        
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                let new_checked = !*checked;
                checked.set(new_checked);
                if let Some(callback) = &on_change {
                    callback.emit(new_checked);
                }
            }
        })
    };
    
    let class = use_memo(
        props.class.clone(),
        |class| {
            let base_class = checkbox_class();
            match class {
                Some(additional) => format!("{} {}", base_class, additional),
                None => base_class.to_string(),
            }
        },
    );

    let child_props = CheckboxChildProps {
        node_ref: props.node_ref.clone(),
        class: (*class).clone(),
        id: props.id.clone(),
        style: props.style.clone(),
        disabled: props.disabled,
        r#type: "button".to_string(),
        role: "checkbox".to_string(),
        aria_checked: if *checked { "true" } else { "false" }.to_string(),
        data_state: if *checked { "checked" } else { "unchecked" }.to_string(),
        onclick: handle_click.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        html! {
            <button
                ref={props.node_ref.clone()}
                class={(*class).clone()}
                id={props.id.clone()}
                style={props.style.clone()}
                disabled={props.disabled}
                type="button"
                role="checkbox"
                aria-checked={if *checked { "true" } else { "false" }}
                data-state={if *checked { "checked" } else { "unchecked" }}
                onclick={handle_click}
            >
                {
                    if *checked {
                        html! {
                            <svg
                                class="h-4 w-4"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path d="M20 6 9 17l-5-5"/>
                            </svg>
                        }
                    } else {
                        props.children.clone()
                    }
                }
            </button>
        }
    }
}