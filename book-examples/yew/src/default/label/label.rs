use yew::prelude::*;

use crate::default::components::ui::label::Label;

#[function_component]
pub fn LabelDemo() -> Html {
    html! {
        <div>
            <div class="flex items-center space-x-2">
                // TODO
                // <Checkbox id="terms" />
                <Label r#for="terms">{"Accept terms and conditions"}</Label>
            </div>
        </div>
    }
}
