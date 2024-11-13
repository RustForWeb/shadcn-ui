#[allow(clippy::module_inception)]
mod breadcrumb;
mod breadcrumb_dropdown;
mod breadcrumb_ellipsis;
mod breadcrumb_link;
mod breadcrumb_responsive;
mod breadcrumb_separator;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum BreadcrumbRoute {
    #[at("/default/")]
    Root,
    #[at("/default/dropdown")]
    Dropdown,
    #[at("/default/ellipsis")]
    Ellipsis,
    #[at("/default/link")]
    Link,
    #[at("/default/responsive")]
    Responsive,
    #[at("/default/separator")]
    Separator,
}

pub fn render(route: BreadcrumbRoute) -> Html {
    match route {
        BreadcrumbRoute::Root => html! { <breadcrumb::BreadcrumbDemo /> },
        BreadcrumbRoute::Dropdown => html! { <breadcrumb_dropdown::BreadcrumbDropdown /> },
        BreadcrumbRoute::Ellipsis => html! { <breadcrumb_ellipsis::BreadcrumbEllipsisDemo /> },
        BreadcrumbRoute::Link => html! { <breadcrumb_link::BreadcrumbLinkDemo /> },
        BreadcrumbRoute::Responsive => html! { <breadcrumb_responsive::BreadcrumbResponsive /> },
        BreadcrumbRoute::Separator => html! { <breadcrumb_separator::BreadcrumbSeparatorDemo /> },
    }
}
