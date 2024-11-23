use leptos::prelude::*;

use crate::new_york::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn ButtonOutline() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Outline}>{"Outline"}</Button>
    }
}
