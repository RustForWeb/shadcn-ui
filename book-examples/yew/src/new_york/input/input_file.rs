use yew::prelude::*;

use crate::new_york::components::ui::{input::Input, label::Label};

#[function_component]
pub fn InputFile() -> Html {
    html! {
        <div class="grid w-full max-w-sm items-center gap-1.5">
            <Label r#for="picture">{"Picture"}</Label>
            <Input id="picture" r#type="file" />
        </div>
    }
}
