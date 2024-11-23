use leptos::prelude::*;

use crate::default::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn ButtonOutline() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Outline}>"Outline"</Button>
    }
}
