use leptos::prelude::*;

use crate::new_york::components::ui::input::{Input, InputType};

#[component]
pub fn InputDisabledDemo() -> impl IntoView {
    view! {
        <Input disabled=true r#type=InputType::Email placeholder="Email" />
    }
}
