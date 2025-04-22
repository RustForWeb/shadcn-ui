use leptos::prelude::*;

use crate::default::components::ui::textarea::Textarea;

#[component]
pub fn TextareaDemo() -> impl IntoView {
    view! {
        <Textarea placeholder="Type your message here." />
    }
}
