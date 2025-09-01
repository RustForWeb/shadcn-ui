use leptos::prelude::*;

use crate::default::components::ui::input::Input;

#[component]
pub fn InputExample() -> impl IntoView {
    let (value, set_value) = create_signal(String::new());

    let handle_change = Callback::new(move |new_value: String| {
        set_value.set(new_value);
    });

    view! {
        <div class="grid w-full max-w-sm items-center gap-1.5">
            <Input
                value=value
                on_change=handle_change
                placeholder="Enter your email"
                input_type="email"
            />
        </div>
    }
}
