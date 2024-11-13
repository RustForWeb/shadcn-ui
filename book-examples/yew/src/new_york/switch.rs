#[allow(clippy::module_inception)]
mod switch;
mod switch_form;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum SwitchRoute {
    #[at("/new-york/")]
    Root,
    #[at("/new-york/form")]
    Form,
}

pub fn render(route: SwitchRoute) -> Html {
    match route {
        SwitchRoute::Root => html! { <switch::SwitchDemo /> },
        SwitchRoute::Form => html! { <switch_form::SwitchForm /> },
    }
}
