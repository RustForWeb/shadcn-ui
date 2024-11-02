#[allow(clippy::module_inception)]
mod pagination;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum PaginationRoute {
    #[at("/new-york/")]
    Root,
}

pub fn render(route: PaginationRoute) -> Html {
    match route {
        PaginationRoute::Root => html! { <pagination::PaginationDemo /> },
    }
}
