use yew::prelude::*;

use crate::new_york::components::ui::button::{Button, ButtonVariant};

#[function_component]
pub fn ButtonOutline() -> Html {
    html! {
        <Button variant={ButtonVariant::Outline}>{"Outline"}</Button>
    }
}
