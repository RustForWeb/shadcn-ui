use yew::prelude::*;

use crate::new_york::components::ui::{input::Input, label::Label};

#[function_component]
pub fn InputWithLabel() -> Html {
    html! {
        <div class="grid w-full max-w-sm items-center gap-1.5">
            <Label r#for="email">{"Email"}</Label>
            <Input r#type="email" id="email" placeholder="Email" />
        </div>
    }
}
