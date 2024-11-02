use yew::prelude::*;

use crate::new_york::components::ui::button::{Button, ButtonVariant};

#[function_component]
pub fn ButtonDestructive() -> Html {
    html! {
        <Button variant={ButtonVariant::Destructive}>{"Destructive"}</Button>
    }
}
