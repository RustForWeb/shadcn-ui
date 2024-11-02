use yew::prelude::*;

use crate::new_york::components::ui::button::{Button, ButtonVariant};

#[function_component]
pub fn ButtonLink() -> Html {
    html! {
        <Button variant={ButtonVariant::Link}>{"Link"}</Button>
    }
}
