#[allow(clippy::module_inception)]
mod separator;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum SeparatorRoute {
    #[at("/new-york/")]
    Root,
}

pub fn render(route: SeparatorRoute) -> Html {
    match route {
        SeparatorRoute::Root => html! { <separator::SeparatorDemo /> },
    }
}
