#[allow(clippy::module_inception)]
mod alert;
mod alert_destructive;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn AlertRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/alert") view=Outlet>
            <Route path=path!("/") view=alert::AlertDemo />
            <Route path=path!("/destructive") view=alert_destructive::AlertDestructive />
        </ParentRoute>
    }
    .into_inner()
}
