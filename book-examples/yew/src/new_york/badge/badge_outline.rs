use yew::prelude::*;

use crate::new_york::components::ui::badge::{Badge, BadgeVariant};

#[function_component]
pub fn BadgeOutline() -> Html {
    html! {
        <Badge variant={BadgeVariant::Outline}>{"Outline"}</Badge>
    }
}
