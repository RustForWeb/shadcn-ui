use leptos::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn ButtonOutline() -> impl IntoView {
    view! {
        <Button variant=ButtonVariant::Outline>Outline</Button>
    }
}
