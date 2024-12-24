use leptos::prelude::*;

use crate::new_york::components::ui::input::{Input, InputType};

#[component]
pub fn InputFileDemo() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm items-center gap-1.5">
            <label r#for="picture">"Picture"</label>
            <Input id="picture" r#type=InputType::File />
        </div>
    }
}
