use leptos::{ev::MouseEvent, prelude::*};
use leptos_style::Style;

// Static classes for better compilation compatibility
const COMPONENT_CLASS: &str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

#[component]
pub fn ComponentName(
    /// The value of the input
    #[prop(into, optional)] value: MaybeProp<String>,
    
    /// Callback when value changes
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    
    /// Placeholder text
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    
    /// Whether the input is disabled
    #[prop(into, optional)] disabled: Signal<bool>,
    
    /// Input type
    #[prop(into, optional)] input_type: MaybeProp<String>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let handle_input = {
        let on_change = on_change.clone();
        move |event: Event| {
            if let Some(callback) = &on_change {
                let target = event.target().unwrap();
                let input = target.unchecked_into::<web_sys::HtmlInputElement>();
                callback.run(input.value());
            }
        }
    };

    let computed_class = Signal::derive(move || {
        format!(
            "{} {}",
            COMPONENT_CLASS,
            class.get().unwrap_or_default()
        )
    });

    view! {
        <input
            type=input_type.get().unwrap_or_else(|| "text".to_string())
            value=value.get().unwrap_or_default()
            placeholder=placeholder.get().unwrap_or_default()
            disabled=disabled
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            on:input=handle_input
        />
    }
}
