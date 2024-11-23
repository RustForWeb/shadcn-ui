use leptos::prelude::*;

use crate::default::components::ui::button::Button;

#[component]
pub fn ButtonDemo() -> impl IntoView {
    view! {
        <Button>"Button"</Button>
    }
}
