use yew::prelude::*;
use yew_style::Style;
use web_sys::MouseEvent;

// Static classes for better compilation compatibility
const RADIO_GROUP_CLASS: &str = "grid gap-2";
const RADIO_ITEM_CLASS: &str = "aspect-square h-4 w-4 rounded-full border border-primary text-primary ring-offset-background focus:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";
const RADIO_INDICATOR_CLASS: &str = "flex items-center justify-center";
const RADIO_INDICATOR_DOT_CLASS: &str = "h-2.5 w-2.5 rounded-full bg-current";

#[derive(PartialEq, Properties)]
pub struct RadioGroupProps {
    /// Currently selected value
    #[prop_or_default]
    pub value: Option<String>,
    
    /// Callback when value changes
    #[prop_or_default]
    pub on_value_change: Option<Callback<String>>,
    
    /// Whether the radio group is disabled
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
    pub children: Html,
}

#[derive(PartialEq, Properties)]
pub struct RadioGroupItemProps {
    /// The value of this radio item
    pub value: String,
    
    /// Whether this item is disabled
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
    pub children: Html,
}

#[function_component]
pub fn RadioGroup(props: &RadioGroupProps) -> Html {
    let selected_value = use_state(|| props.value.clone());
    
    let on_item_select = {
        let selected_value = selected_value.clone();
        let on_value_change = props.on_value_change.clone();
        
        Callback::from(move |value: String| {
            selected_value.set(Some(value.clone()));
            if let Some(callback) = &on_value_change {
                callback.emit(value);
            }
        })
    };
    
    let _context = use_context::<RadioGroupContext>().unwrap_or_default();
    let new_context = RadioGroupContext {
        selected_value: (*selected_value).clone(),
        on_item_select,
        disabled: props.disabled,
    };
    
    let computed_class = format!(
        "{} {}",
        RADIO_GROUP_CLASS,
        props.class.as_deref().unwrap_or_default()
    );
    
    html! {
        <ContextProvider<RadioGroupContext> context={new_context}>
            <div
                class={computed_class}
                id={props.id.clone()}
                style={props.style.clone()}
                role="radiogroup"
            >
                {props.children.clone()}
            </div>
        </ContextProvider<RadioGroupContext>>
    }
}

#[derive(Clone, PartialEq)]
struct RadioGroupContext {
    selected_value: Option<String>,
    on_item_select: Callback<String>,
    disabled: bool,
}

impl Default for RadioGroupContext {
    fn default() -> Self {
        Self {
            selected_value: None,
            on_item_select: Callback::noop(),
            disabled: false,
        }
    }
}

#[function_component]
pub fn RadioGroupItem(props: &RadioGroupItemProps) -> Html {
    let context = use_context::<RadioGroupContext>().unwrap_or_default();
    let is_selected = context.selected_value.as_ref() == Some(&props.value);
    let is_disabled = props.disabled || context.disabled;
    
    let handle_click = {
        let value = props.value.clone();
        let on_select = context.on_item_select.clone();
        
        Callback::from(move |_: MouseEvent| {
            if !is_disabled {
                on_select.emit(value.clone());
            }
        })
    };
    
    let computed_class = format!(
        "{} {}",
        RADIO_ITEM_CLASS,
        props.class.as_deref().unwrap_or_default()
    );
    
    html! {
        <button
            type="button"
            role="radio"
            aria-checked={is_selected.to_string()}
            data-state={if is_selected { "checked" } else { "unchecked" }}
            data-disabled={is_disabled.to_string()}
            class={computed_class}
            id={props.id.clone()}
            style={props.style.clone()}
            disabled={is_disabled}
            onclick={handle_click}
        >
            <div class={RADIO_INDICATOR_CLASS}>
                {
                    if is_selected {
                        html! {
                            <div class={RADIO_INDICATOR_DOT_CLASS} />
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
            {props.children.clone()}
        </button>
    }
}