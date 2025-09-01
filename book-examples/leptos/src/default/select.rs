use leptos::prelude::*;

use crate::default::components::ui::select::{
    Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
};

#[component]
pub fn SelectExample() -> impl IntoView {
    let (value, set_value) = create_signal(String::new());

    let handle_value_change = Callback::new(move |new_value: String| {
        set_value.set(new_value);
    });

    view! {
        <Select value=value on_value_change=handle_value_change>
            <SelectTrigger class="w-[180px]">
                <SelectValue placeholder="Select a fruit" />
            </SelectTrigger>
            <SelectContent>
                <SelectItem value="apple">"Apple"</SelectItem>
                <SelectItem value="banana">"Banana"</SelectItem>
                <SelectItem value="blueberry">"Blueberry"</SelectItem>
                <SelectItem value="grapes">"Grapes"</SelectItem>
                <SelectItem value="pineapple">"Pineapple"</SelectItem>
            </SelectContent>
        </Select>
    }
}
