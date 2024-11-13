#[allow(clippy::module_inception)]
mod textarea;
mod textarea_disabled;
mod textarea_form;
mod textarea_with_button;
mod textarea_with_label;
mod textarea_with_text;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum TextareaRoute {
    #[at("/new-york/")]
    Root,
    #[at("/new-york/disabled")]
    Disabled,
    #[at("/new-york/form")]
    Form,
    #[at("/new-york/with-button")]
    WithButton,
    #[at("/new-york/with-label")]
    WithLabel,
    #[at("/new-york/with-text")]
    WithText,
}

pub fn render(route: TextareaRoute) -> Html {
    match route {
        TextareaRoute::Root => html! { <textarea::TextareaDemo /> },
        TextareaRoute::Disabled => html! { <textarea_disabled::TextareaDisabled /> },
        TextareaRoute::Form => html! { <textarea_form::TextareaForm /> },
        TextareaRoute::WithButton => html! { <textarea_with_button::TextareaWithButton /> },
        TextareaRoute::WithLabel => html! { <textarea_with_label::TextareaWithLabel /> },
        TextareaRoute::WithText => html! { <textarea_with_text::TextareaWithText /> },
    }
}
