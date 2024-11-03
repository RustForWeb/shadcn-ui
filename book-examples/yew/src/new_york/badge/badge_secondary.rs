use yew::prelude::*;

use crate::new_york::components::ui::badge::{Badge, BadgeVariant};

#[function_component]
pub fn BadgeSecondary() -> Html {
    html! {
        <Badge variant={BadgeVariant::Secondary}>{"Secondary"}</Badge>
    }
}
