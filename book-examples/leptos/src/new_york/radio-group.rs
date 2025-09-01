#[allow(clippy::module_inception)]
mod radio_group;

use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes,
    components::{Outlet, ParentRoute, Route},
    path,
};

#[component(transparent)]
pub fn RadioGroupRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/radio-group") view=Outlet>
            <Route path=path!("/") view=radio_group::RadioGroupExample />
        </ParentRoute>
    }
    .into_inner()
}
