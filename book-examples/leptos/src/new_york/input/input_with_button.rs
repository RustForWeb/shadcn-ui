use leptos::prelude::*;

use crate::new_york::components::ui::button::{Button, ButtonVariant};
use crate::new_york::components::ui::input::{Input, InputType};
#[component]
pub fn InputWithButtonDemo() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-sm items-center space-x-2">
            <Input r#type=InputType::Email placeholder="Email" />
            <Button r#type="Submit" variant=ButtonVariant::Secondary>"Subscribe"</Button>
        </div>
    }
}
