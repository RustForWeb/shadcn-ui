#[allow(clippy::module_inception)]
mod label;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum LabelRoute {
    #[at("/default/")]
    Root,
}

pub fn render(route: LabelRoute) -> Html {
    match route {
        LabelRoute::Root => html! { <label::LabelDemo /> },
    }
}
