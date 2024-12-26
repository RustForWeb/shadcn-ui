mod components;

#[cfg(feature = "alert")]
mod alert;

#[cfg(feature = "breadcrumb")]
mod breadcrumb;

#[cfg(feature = "button")]
mod button;

#[cfg(feature = "card")]
mod card;

#[cfg(feature = "pagination")]
mod pagination;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn NewYork() -> impl MatchNestedRoutes + Clone {
    let children = (
        #[cfg(feature = "alert")]
        {
            component_view(self::alert::AlertRoutes, ())
        },
        #[cfg(feature = "breadcrumb")]
        {
            component_view(self::breadcrumb::BreadcrumbRoutes, ())
        },
        #[cfg(feature = "button")]
        {
            component_view(self::button::ButtonRoutes, ())
        },
        #[cfg(feature = "card")]
        {
            component_view(self::card::CardRoutes, ())
        },
        #[cfg(feature = "pagination")]
        {
            component_view(self::pagination::PaginationRoutes, ())
        },
    );

    view! {
        <ParentRoute path=path!("new-york") view=Outlet children=ToChildren::to_children(move || children) />
    }
    .into_inner()
}
