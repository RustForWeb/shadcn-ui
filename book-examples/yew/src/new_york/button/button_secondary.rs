use yew::prelude::*;

use crate::new_york::components::ui::button::{Button, ButtonVariant};

#[function_component]
pub fn ButtonSecondary() -> Html {
    html! {
        <Button variant={ButtonVariant::Secondary}>{"Secondary"}</Button>
    }
}
