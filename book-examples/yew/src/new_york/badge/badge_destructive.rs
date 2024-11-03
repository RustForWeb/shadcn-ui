use yew::prelude::*;

use crate::new_york::components::ui::badge::{Badge, BadgeVariant};

#[function_component]
pub fn BadgeDestructive() -> Html {
    html! {
        <Badge variant={BadgeVariant::Destructive}>{"Destructive"}</Badge>
    }
}
