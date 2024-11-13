use yew::prelude::*;

use crate::new_york::components::ui::{button::Button, textarea::Textarea};

#[function_component]
pub fn TextareaWithButton() -> Html {
    html! {
        <div class="grid w-full gap-2">
            <Textarea placeholder="Type your message here." />
            <Button>{"Send message"}</Button>
        </div>
    }
}
