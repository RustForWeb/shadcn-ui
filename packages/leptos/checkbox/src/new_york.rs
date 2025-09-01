use leptos::{ev::Event, prelude::*};
use leptos_style::Style;
use leptos::wasm_bindgen::JsCast;

const CHECKBOX_CLASS: &str = "h-4 w-4 shrink-0 rounded-sm border border-primary ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground";

#[component]
pub fn Checkbox(
    #[prop(into, optional)] checked: Signal<bool>,
    #[prop(into, optional)] on_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let handle_change = {
        let on_change = on_change.clone();
        move |event: Event| {
            if let Some(callback) = &on_change {
                let target = event.target().unwrap();
                let input = target.unchecked_into::<web_sys::HtmlInputElement>();
                callback.run(input.checked());
            }
        }
    };

    let computed_class = Signal::derive(move || {
        format!("{} {}", CHECKBOX_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <input
            type="checkbox"
            checked=checked
            disabled=disabled
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            on:change=handle_change
        />
    }
}
