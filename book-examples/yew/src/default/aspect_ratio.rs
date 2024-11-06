#[allow(clippy::module_inception)]
mod aspect_ratio;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum AspectRatioRoute {
    #[at("/default/")]
    Root,
}

pub fn render(route: AspectRatioRoute) -> Html {
    match route {
        AspectRatioRoute::Root => html! { <aspect_ratio::AspectRatioDemo /> },
    }
}
