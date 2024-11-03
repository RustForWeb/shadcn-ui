#[allow(clippy::module_inception)]
mod badge;
mod badge_destructive;
mod badge_outline;
mod badge_secondary;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum BadgeRoute {
    #[at("/default/")]
    Root,
    #[at("/default/destructive")]
    Destructive,
    #[at("/default/outline")]
    Outline,
    #[at("/default/secondary")]
    Secondary,
}

pub fn render(route: BadgeRoute) -> Html {
    match route {
        BadgeRoute::Root => html! { <badge::BadgeDemo /> },
        BadgeRoute::Destructive => html! { <badge_destructive::BadgeDestructive /> },
        BadgeRoute::Outline => html! { <badge_outline::BadgeOutline /> },
        BadgeRoute::Secondary => html! { <badge_secondary::BadgeSecondary /> },
    }
}
