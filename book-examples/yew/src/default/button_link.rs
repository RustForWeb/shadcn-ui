use yew::prelude::*;

use super::components::ui::button::{Button, ButtonVariant};

#[function_component]
pub fn ButtonLink() -> Html {
    html! {
        <Button variant={ButtonVariant::Link}>{"Link"}</Button>
    }
}
