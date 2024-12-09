#[allow(clippy::module_inception)]
mod alert;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn AlertRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("") view=Outlet>
            <Route path=path!("/") view=alert::AlertDemo />
        </ParentRoute>
    }
    .into_inner()
}
