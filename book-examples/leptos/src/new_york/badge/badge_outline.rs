use leptos::prelude::*;

use crate::new_york::components::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn BadgeOutline() -> impl IntoView {
    view! {
        <Badge variant={BadgeVariant::Outline}>{"Outline"}</Badge>
    }
}
