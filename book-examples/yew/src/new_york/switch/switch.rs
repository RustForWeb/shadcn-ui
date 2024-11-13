use yew::prelude::*;

use crate::new_york::components::ui::{label::Label, switch::Switch};

#[function_component]
pub fn SwitchDemo() -> Html {
    html! {
        <div class="flex items-center space-x-2">
            <Switch id="airplane-mode" />
            <Label r#for="airplane-mode">{"Airplane Mode"}</Label>
        </div>
    }
}
