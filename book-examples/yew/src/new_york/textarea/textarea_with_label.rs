use yew::prelude::*;

use crate::new_york::components::ui::{label::Label, textarea::Textarea};

#[function_component]
pub fn TextareaWithLabel() -> Html {
    html! {
        <div class="grid w-full gap-1.5">
            <Label r#for="message">{"Your message"}</Label>
            <Textarea placeholder="Type your message here." id="message" />
        </div>
    }
}
