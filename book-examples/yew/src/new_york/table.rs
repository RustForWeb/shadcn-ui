#[allow(clippy::module_inception)]
mod table;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum TableRoute {
    #[at("/new-york/")]
    Root,
}

pub fn render(route: TableRoute) -> Html {
    match route {
        TableRoute::Root => html! { <table::TableDemo /> },
    }
}
