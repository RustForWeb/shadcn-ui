#[allow(clippy::module_inception)]
mod alert;
mod alert_destructive;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum AlertRoute {
    #[at("/default/")]
    Root,
    #[at("/default/destructive")]
    Destructive,
}

pub fn render(route: AlertRoute) -> Html {
    match route {
        AlertRoute::Root => html! { <alert::AlertDemo /> },
        AlertRoute::Destructive => html! { <alert_destructive::AlertDestructive /> },
    }
}
