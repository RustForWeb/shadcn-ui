use yew::prelude::*;

use crate::new_york::components::ui::badge::Badge;

#[function_component]
pub fn BadgeDemo() -> Html {
    html! {
        <Badge>{"Badge"}</Badge>
    }
}
