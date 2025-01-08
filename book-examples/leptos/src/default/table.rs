#[allow(clippy::module_inception)]
mod table;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn TableRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/table") view=Outlet>
            <Route path=path!("/") view=table::TableDemo />
        </ParentRoute>
    }
    .into_inner()
}