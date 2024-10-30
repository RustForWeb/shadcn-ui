use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn ButtonDestructive() -> impl IntoView {
    view! {
        <Button variant=ButtonVariant::Destructive>Destructive</Button>
    }
}
