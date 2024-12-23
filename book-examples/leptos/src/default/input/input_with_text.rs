use leptos::prelude::*;

use crate::default::components::ui::input::{Input,InputType};

#[component]
pub fn InputWithTextDemo() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm items-center gap-1.5">
            <label r#for="email-2">"Email"</label>
            <Input r#type=InputType::Email id="email-2" placeholder="Email" />
            <p class="text-sm text-muted-foreground">"Enter your email address."</p>
        </div>
    }
}
