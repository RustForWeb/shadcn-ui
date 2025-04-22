use leptos::prelude::*;

use crate::new_york::components::ui::textarea::Textarea;

#[component]
pub fn TextareaWithLabelDemo() -> impl IntoView {
    view! {
        <div class="grid w-full gap-1.5">
            <label r#for="message">"Your message"</label>
            <Textarea placeholder="Type your message here." id="message" />
        </div>
    }
}
