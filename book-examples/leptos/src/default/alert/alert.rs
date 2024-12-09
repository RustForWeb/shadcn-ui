use leptos::prelude::*;

use crate::default::components::ui::alert::Alert;

#[component]
pub fn AlertDemo() -> impl IntoView {
    view! {
        <Alert>"Alert"</Alert>
    }
}
