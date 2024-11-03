use yew::prelude::*;

use crate::default::components::ui::badge::Badge;

#[function_component]
pub fn BadgeDemo() -> Html {
    html! {
        <Badge>{"Badge"}</Badge>
    }
}
