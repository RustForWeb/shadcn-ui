use yew::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[function_component]
pub fn ButtonLink() -> Html {
    html! {
        <Button variant={ButtonVariant::Link}>{"Link"}</Button>
    }
}
