#[allow(clippy::module_inception)]
mod breadcrumb;
mod breadcrumb_dropdown;
mod breadcrumb_ellipsis;
mod breadcrumb_link;
mod breadcrumb_responsive;
mod breadcrumb_separator;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn BreadcrumbRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/breadcrumb") view=Outlet>
            <Route path=path!("/") view=breadcrumb::BreadcrumbDemo />
            <Route path=path!("/dropdown") view=breadcrumb_dropdown::BreadcrumbDropdownDemo />
            <Route path=path!("/ellipsis") view=breadcrumb_ellipsis::BreadcrumbEllipsisDemo/>
            <Route path=path!("/link") view=breadcrumb_link::BreadcrumbLinkDemo/>
            <Route path=path!("/responsive") view=breadcrumb_responsive::BreadcrumbResponsiveDemo/>
            <Route path=path!("/separator") view=breadcrumb_separator::BreadcrumbSeparatorDemo/>
        </ParentRoute>
    }
    .into_inner()
}
