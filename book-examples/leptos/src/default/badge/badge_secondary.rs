use leptos::prelude::*;

use crate::default::components::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn BadgeSecondary() -> impl IntoView {
    view! {
        <Badge variant={BadgeVariant::Secondary}>{"Secondary"}</Badge>
    }
}
