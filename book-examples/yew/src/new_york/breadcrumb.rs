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
    #[at("/new-york/")]
    Root,
    #[at("/new-york/dropdown")]
    Dropdown,
    #[at("/new-york/ellipsis")]
    Ellipsis,
    #[at("/new-york/link")]
    Link,
    #[at("/new-york/responsive")]
    Responsive,
    #[at("/new-york/separator")]
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
