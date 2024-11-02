use yew::prelude::*;

use crate::default::components::ui::button::{Button, ButtonVariant};

#[function_component]
pub fn ButtonSecondary() -> Html {
    html! {
        <Button variant={ButtonVariant::Secondary}>{"Secondary"}</Button>
    }
}
