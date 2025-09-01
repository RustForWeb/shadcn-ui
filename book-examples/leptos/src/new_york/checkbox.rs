use leptos::prelude::*;

use crate::new_york::components::ui::checkbox::Checkbox;

#[component]
pub fn CheckboxExample() -> impl IntoView {
    let (checked, set_checked) = create_signal(false);

    let handle_change = Callback::new(move |new_checked: bool| {
        set_checked.set(new_checked);
    });

    view! {
        <div class="flex items-center space-x-2">
            <Checkbox
                checked=checked
                on_change=handle_change
                id="terms"
            />
            <label
                for="terms"
                class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >
                "Accept terms and conditions"
            </label>
        </div>
    }
}
