use yew::prelude::*;

use super::components::ui::button::{Button, ButtonVariant};

#[function_component]
pub fn ButtonGhost() -> Html {
    html! {
        <Button variant={ButtonVariant::Ghost}>{"Ghost"}</Button>
    }
}
