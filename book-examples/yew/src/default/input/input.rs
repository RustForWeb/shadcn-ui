use yew::prelude::*;

use crate::default::components::ui::input::Input;

#[function_component]
pub fn InputDemo() -> Html {
    html! {
        <Input r#type="email" placeholder="Email" />
    }
}
