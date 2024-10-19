use yew::prelude::*;

use crate::components::ui::button::Button;

#[function_component]
pub fn ButtonDemo() -> Html {
    html! {
        <Button>{"Button"}</Button>
    }
}
