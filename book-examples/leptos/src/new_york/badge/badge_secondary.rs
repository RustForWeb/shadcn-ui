use leptos::prelude::*;

use crate::new_york::components::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn BadgeSecondary() -> impl IntoView {
    view! {
        <Badge variant={BadgeVariant::Secondary}>{"Secondary"}</Badge>
    }
}
