use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[component]
pub fn InputOtp(
    #[prop(into, optional)] length: MaybeProp<usize>,
    #[prop(into, optional)] value: RwSignal<String>,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let len = length.get().unwrap_or(6);

    let handle_input = {
        let value = value.clone();
        let on_change = on_change.clone();
        Callback::new(move |new_val: String| {
            value.set(new_val.clone());
            if let Some(cb) = &on_change { cb.run(new_val); }
        })
    };

    let classes = Signal::derive(move || class.get().unwrap_or_default());

    view! {
        <div class=classes>
            <input
                value=move || value.get()
                on:input=move |ev| {
                    let target: HtmlInputElement = ev.target().unwrap().unchecked_into();
                    let mut v = target.value();
                    v.truncate(len);
                    handle_input.run(v);
                }
            />
        </div>
    }
}

pub mod prelude { pub use super::InputOtp; }

#[cfg(test)]
mod tests;
