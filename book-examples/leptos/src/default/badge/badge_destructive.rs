use leptos::prelude::*;

use crate::default::components::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn BadgeDestructive() -> impl IntoView {
    view! {
        <Badge variant={BadgeVariant::Destructive}>{"Destructive"}</Badge>
    }
}
