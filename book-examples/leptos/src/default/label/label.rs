use crate::default::components::ui::label::Label;
use leptos::prelude::*;

#[component]
pub fn LabelDemo() -> impl IntoView {
    view! {
        <div>
            <div class="flex items-center space-x-2">
                // TODO
                // <Checkbox id="terms" />
                <Label
                    r#for="terms"
                >
                    "I agree to the terms and conditions"
                </Label>
            </div>
        </div>
    }
}
