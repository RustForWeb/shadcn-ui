#[allow(clippy::module_inception)]
mod card;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn CardRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/card") view=Outlet>
            <Route path=path!("/") view=card::CardDemo />
        </ParentRoute>
    }
    .into_inner()
}
