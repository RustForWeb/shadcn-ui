use leptos::prelude::*;

use crate::new_york::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn ButtonLink() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Link}>{"Link"}</Button>
    }
}
