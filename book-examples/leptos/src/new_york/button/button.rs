use leptos::prelude::*;

use crate::new_york::components::ui::button::Button;

#[component]
pub fn ButtonDemo() -> impl IntoView {
    view! {
        <Button>"Button"</Button>
    }
}
