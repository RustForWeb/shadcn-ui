use leptos::prelude::*;

use crate::new_york::components::ui::textarea::Textarea;

#[component]
pub fn TextareaDemo() -> impl IntoView {
    view! {
        <Textarea placeholder="Type your message here." />
    }
}
