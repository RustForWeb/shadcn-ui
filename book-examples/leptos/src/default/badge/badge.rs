use leptos::prelude::*;

use crate::default::components::ui::badge::Badge;

#[component]
pub fn BadgeDemo() -> impl IntoView {
    view! {
        <Badge>{"Badge"}</Badge>
    }
}
