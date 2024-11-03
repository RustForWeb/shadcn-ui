#[allow(clippy::module_inception)]
mod input;
mod input_disabled;
mod input_file;
mod input_form;
mod input_with_button;
mod input_with_label;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum InputRoute {
    #[at("/new-york/")]
    Root,
    #[at("/new-york/disabled")]
    Disabled,
    #[at("/new-york/file")]
    File,
    #[at("/new-york/form")]
    Form,
    #[at("/new-york/with-button")]
    WithButton,
    #[at("/new-york/with-label")]
    WithLabel,
}

pub fn render(route: InputRoute) -> Html {
    match route {
        InputRoute::Root => html! { <input::InputDemo /> },
        InputRoute::Disabled => html! { <input_disabled::InputDisabled /> },
        InputRoute::File => html! { <input_file::InputFile /> },
        InputRoute::Form => html! { <input_form::InputForm /> },
        InputRoute::WithButton => html! { <input_with_button::InputWithButton /> },
        InputRoute::WithLabel => html! { <input_with_label::InputWithLabel /> },
    }
}
