use yew::prelude::*;

use crate::new_york::components::ui::input::Input;

#[function_component]
pub fn InputDisabled() -> Html {
    html! {
        <Input disabled=true r#type="email" placeholder="Email" />
    }
}
