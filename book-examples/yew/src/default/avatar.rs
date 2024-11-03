#[allow(clippy::module_inception)]
mod avatar;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum AvatarRoute {
    #[at("/default/")]
    Root,
}

pub fn render(route: AvatarRoute) -> Html {
    match route {
        AvatarRoute::Root => html! { <avatar::AvatarDemo /> },
    }
}
