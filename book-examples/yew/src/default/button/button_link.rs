use yew::prelude::*;

use crate::default::components::ui::button::{Button, ButtonVariant};

#[function_component]
pub fn ButtonLink() -> Html {
    html! {
        <Button variant={ButtonVariant::Link}>{"Link"}</Button>
    }
}
