use yew::prelude::*;

use crate::new_york::components::ui::button::Button;

#[function_component]
pub fn ButtonDemo() -> Html {
    html! {
        <Button>{"Button"}</Button>
    }
}
