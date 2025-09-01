use yew::prelude::*;
use yew_router::prelude::*;

mod dialog;

#[derive(Clone, Routable, PartialEq)]
pub enum DialogRoute {
    #[at("/dialog")]
    Dialog,
}

pub fn render(route: DialogRoute) -> Html {
    match route {
        DialogRoute::Dialog => html! {
            <div class="container mx-auto py-8">
                <dialog::DialogExample />
            </div>
        },
    }
}
