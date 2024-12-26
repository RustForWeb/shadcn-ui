#[allow(clippy::module_inception)]
mod pagination;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn PaginationRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/pagination") view=Outlet>
            <Route path=path!("/") view=pagination::PaginationDemo />
        </ParentRoute>
    }
    .into_inner()
}
