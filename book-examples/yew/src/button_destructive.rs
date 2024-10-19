use yew::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[function_component]
pub fn ButtonDestructive() -> Html {
    html! {
        <Button variant={ButtonVariant::Destructive}>{"Destructive"}</Button>
    }
}
