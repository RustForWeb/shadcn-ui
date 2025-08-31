use leptos::{ev::MouseEvent, prelude::*};
use leptos_style::Style;

// New York variant with subtle styling differences
const RADIO_GROUP_CLASS: &str = "grid gap-2";
const RADIO_ITEM_CLASS: &str = "aspect-square h-4 w-4 rounded-full border border-primary text-primary shadow focus:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
const RADIO_INDICATOR_CLASS: &str = "flex items-center justify-center";
const RADIO_INDICATOR_DOT_CLASS: &str = "h-2.5 w-2.5 rounded-full bg-current";

#[component]
pub fn RadioGroup(
    /// Currently selected value
    #[prop(into, optional)] value: MaybeProp<String>,
    
    /// Callback when value changes
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    
    /// Whether the radio group is disabled
    #[prop(into, optional)] disabled: Signal<bool>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(optional, into)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let selected_value = RwSignal::new(value.get_untracked());
    
    let on_item_select = {
        let selected_value = selected_value.clone();
        let on_value_change = on_value_change.clone();
        
        Callback::new(move |value: String| {
            selected_value.set(Some(value.clone()));
            if let Some(callback) = &on_value_change {
                callback.run(value);
            }
        })
    };
    
    let context = RadioGroupContext {
        selected_value: selected_value.read_only(),
        on_item_select,
        disabled,
    };
    
    let computed_class = Signal::derive(move || {
        format!(
            "{} {}",
            RADIO_GROUP_CLASS,
            class.get().unwrap_or_default()
        )
    });
    
    provide_context(context);
    
    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            role="radiogroup"
        >
            {children.map(|c| c()).unwrap_or_else(|| view! { <div></div> }.into_any())}
        </div>
    }
}

#[derive(Clone)]
struct RadioGroupContext {
    selected_value: ReadSignal<Option<String>>,
    on_item_select: Callback<String>,
    disabled: Signal<bool>,
}

#[component]
pub fn RadioGroupItem(
    /// The value of this radio item
    #[prop(into)] value: String,
    
    /// Whether this item is disabled
    #[prop(into, optional)] disabled: Signal<bool>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(optional, into)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = use_context::<RadioGroupContext>().expect("RadioGroupItem must be used within RadioGroup");
    
    let value_clone = value.clone();
    let is_selected = Signal::derive(move || {
        context.selected_value.get().as_ref() == Some(&value_clone)
    });
    
    let is_disabled = Signal::derive(move || {
        disabled.get() || context.disabled.get()
    });
    
    let handle_click = {
        let value = value.clone();
        let on_select = context.on_item_select.clone();
        
        move |_: MouseEvent| {
            if !is_disabled.get() {
                on_select.run(value.clone());
            }
        }
    };
    
    let computed_class = Signal::derive(move || {
        format!(
            "{} {}",
            RADIO_ITEM_CLASS,
            class.get().unwrap_or_default()
        )
    });
    
    let aria_checked = Signal::derive(move || {
        is_selected.get().to_string()
    });
    
    let data_state = Signal::derive(move || {
        if is_selected.get() { "checked" } else { "unchecked" }
    });
    
    let data_disabled = Signal::derive(move || {
        is_disabled.get().to_string()
    });
    
    view! {
        <button
            type="button"
            role="radio"
            aria-checked=aria_checked
            data-state=data_state
            data-disabled=data_disabled
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            disabled=is_disabled
            on:click=handle_click
        >
            <div class=RADIO_INDICATOR_CLASS>
                {
                    move || {
                        if is_selected.get() {
                            view! {
                                <div class=RADIO_INDICATOR_DOT_CLASS />
                            }
                        } else {
                            view! { <div class=""></div> }
                        }
                    }
                }
            </div>
            {children.map(|c| c()).unwrap_or_else(|| view! { <div></div> }.into_any())}
        </button>
    }
}
