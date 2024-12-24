use leptos::prelude::*;

use crate::default::components::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn BadgeOutline() -> impl IntoView {
    view! {
        <Badge variant={BadgeVariant::Outline}>{"Outline"}</Badge>
    }
}
