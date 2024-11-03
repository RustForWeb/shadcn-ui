#[allow(clippy::module_inception)]
mod card;
mod card_with_form;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum CardRoute {
    #[at("/new-york/")]
    Root,
    #[at("/new-york/with-form")]
    WithForm,
}

pub fn render(route: CardRoute) -> Html {
    match route {
        CardRoute::Root => html! { <card::CardDemo /> },
        CardRoute::WithForm => html! { <card_with_form::CardWithForm /> },
    }
}
