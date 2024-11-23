use leptos::prelude::*;

use crate::default::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn ButtonLink() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Link}>"Link"</Button>
    }
}
