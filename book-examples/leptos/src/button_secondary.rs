use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn ButtonSecondary() -> impl IntoView {
    view! {
        <Button variant=ButtonVariant::Secondary>Secondary</Button>
    }
}
