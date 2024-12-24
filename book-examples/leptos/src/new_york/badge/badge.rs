use leptos::prelude::*;

use crate::new_york::components::ui::badge::Badge;

#[component]
pub fn BadgeDemo() -> impl IntoView {
    view! {
        <Badge>{"Badge"}</Badge>
    }
}
