use yew::prelude::*;

use crate::default::components::ui::{button::Button, input::Input};

#[function_component]
pub fn InputWithButton() -> Html {
    html! {
        <div class="flex w-full max-w-sm items-center space-x-2">
            <Input r#type="email" placeholder="Email" />
            <Button r#type="submit">{"Subscribe"}</Button>
        </div>
    }
}
