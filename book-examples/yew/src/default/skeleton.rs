#[allow(clippy::module_inception)]
mod skeleton;
mod skeleton_card;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum SkeletonRoute {
    #[at("/default/")]
    Root,
    #[at("/default/card")]
    Card,
}

pub fn render(route: SkeletonRoute) -> Html {
    match route {
        SkeletonRoute::Root => html! { <skeleton::SkeletonDemo /> },
        SkeletonRoute::Card => html! { <skeleton_card::SkeletonCard /> },
    }
}
