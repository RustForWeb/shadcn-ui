use leptos::prelude::*;

use crate::new_york::components::ui::alert::Alert;

#[component]
pub fn AlertDemo() -> impl IntoView {
    view! {
        <Alert>"Alert"</Alert>
    }
}
