use yew::prelude::*;

use crate::default::components::ui::input::Input;

#[function_component]
pub fn InputDisabled() -> Html {
    html! {
        <Input disabled=true r#type="email" placeholder="Email" />
    }
}
