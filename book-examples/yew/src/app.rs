use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum StyleRoute {
    #[at("/default")]
    DefaultRoot,
    #[at("/default/*")]
    Default,
    #[at("/new-york")]
    NewYorkRoot,
    #[at("/new-york/*")]
    NewYork,
}

fn render_style(route: StyleRoute) -> Html {
    match route {
        StyleRoute::DefaultRoot | StyleRoute::Default => crate::default::render(),
        StyleRoute::NewYorkRoot | StyleRoute::NewYork => crate::new_york::render(),
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="flex min-h-[350px] w-full justify-center p-10 items-center">
            <HashRouter>
                <Switch<StyleRoute> render={render_style} />
            </HashRouter>
        </div>
    }
}
