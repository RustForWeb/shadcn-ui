use yew::prelude::*;

use crate::default::components::ui::{input::Input, label::Label};

#[function_component]
pub fn InputWithText() -> Html {
    html! {
        <div class="grid w-full max-w-sm items-center gap-1.5">
            <Label r#for="email-2">{"Email"}</Label>
            <Input r#type="email" id="email-2" placeholder="Email" />
            <p class="text-sm text-muted-foreground">{"Enter your email address."}</p>
        </div>
    }
}
