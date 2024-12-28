use leptos::prelude::*;

use crate::default::components::ui::textarea::Textarea;

#[component]
pub fn TextareaWithTextDemo() -> impl IntoView {
    view! {
        <div class="grid w-full gap-1.5">
            <label r#for="message-2">"Your Message"</label>
            <Textarea placeholder="Type your message here." id="message-2" />
            <p class="text-sm text-muted-foreground">
                {"Your message will be copied to the support team."}
            </p>
        </div>
    }
}
