use leptos::prelude::*;

use crate::default::components::ui::label::Label;

#[component]
pub fn LabelDemo() -> impl IntoView {
    view! {
        <div>
            <div class="flex items-center space-x-2">
                // TODO
                // <Checkbox id="terms" />
                <Label r#for="terms">{"Accept terms and conditions"}</Label>
            </div>
        </div>
    }
}
