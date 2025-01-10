#[allow(clippy::module_inception)]
mod label;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn LabelRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/label") view=Outlet>
            <Route path=path!("/") view=label::LabelDemo />
        </ParentRoute>
    }
    .into_inner()
}
