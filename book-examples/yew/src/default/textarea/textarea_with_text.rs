use yew::prelude::*;

use crate::default::components::ui::{label::Label, textarea::Textarea};

#[function_component]
pub fn TextareaWithText() -> Html {
    html! {
        <div class="grid w-full gap-1.5">
            <Label r#for="message-2">{"Your Message"}</Label>
            <Textarea placeholder="Type your message here." id="message-2" />
            <p class="text-sm text-muted-foreground">
                {"Your message will be copied to the support team."}
            </p>
        </div>
    }
}
