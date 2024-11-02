#[allow(clippy::module_inception)]
mod button;
mod button_as_child;
mod button_destructive;
mod button_ghost;
mod button_icon;
mod button_link;
mod button_loading;
mod button_outline;
mod button_secondary;
mod button_with_icon;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum ButtonRoute {
    #[at("/default/")]
    Root,
    #[at("/default/as-child")]
    AsChild,
    #[at("/default/destructive")]
    Destructive,
    #[at("/default/ghost")]
    Ghost,
    #[at("/default/icon")]
    Icon,
    #[at("/default/link")]
    Link,
    #[at("/default/loading")]
    Loading,
    #[at("/default/outline")]
    Outline,
    #[at("/default/secondary")]
    Secondary,
    #[at("/default/with-icon")]
    WithIcon,
}

pub fn render(route: ButtonRoute) -> Html {
    match route {
        ButtonRoute::Root => html! { <button::ButtonDemo /> },
        ButtonRoute::AsChild => html! { <button_as_child::ButtonAsChild /> },
        ButtonRoute::Destructive => html! { <button_destructive::ButtonDestructive /> },
        ButtonRoute::Ghost => html! { <button_ghost::ButtonGhost /> },
        ButtonRoute::Icon => html! { <button_icon::ButtonIcon /> },
        ButtonRoute::Link => html! { <button_link::ButtonLink /> },
        ButtonRoute::Loading => html! { <button_loading::ButtonLoading /> },
        ButtonRoute::Outline => html! { <button_outline::ButtonOutline /> },
        ButtonRoute::Secondary => html! { <button_secondary::ButtonSecondary /> },
        ButtonRoute::WithIcon => html! { <button_with_icon::ButtonWithIcon /> },
    }
}
