use yew::prelude::*;

use crate::default::components::ui::button::{Button, ButtonVariant};

#[function_component]
pub fn ButtonOutline() -> Html {
    html! {
        <Button variant={ButtonVariant::Outline}>{"Outline"}</Button>
    }
}
