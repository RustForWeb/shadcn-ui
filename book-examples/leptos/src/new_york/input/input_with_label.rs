use leptos::prelude::*;

use crate::new_york::components::ui::input::{Input, InputType};

#[component]
pub fn InputWithLabelDemo() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm items-center gap-1.5">
            <label r#for="email">"Email"</label>
            <Input r#type=InputType::Email id="email" placeholder="Email" />
        </div>
    }
}
