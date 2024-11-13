use yew::prelude::*;

use crate::default::components::ui::textarea::Textarea;

#[function_component]
pub fn TextareaDisabled() -> Html {
    html! {
        <Textarea placeholder="Type your message here." disabled=true />
    }
}
