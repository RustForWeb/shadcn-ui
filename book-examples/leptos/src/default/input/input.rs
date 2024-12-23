use leptos::prelude::*;

use crate::default::components::ui::input::{Input, InputType };

#[component]
pub fn InputDemo() -> impl IntoView {
    view! {
        <Input r#type=InputType::Email placeholder="Email" />
    }
}
