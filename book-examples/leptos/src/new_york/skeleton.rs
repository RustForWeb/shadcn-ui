#[allow(clippy::module_inception)]
mod skeleton;
mod skeleton_card;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn SkeletonRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/skeleton") view=Outlet>
            <Route path=path!("/") view=skeleton::SkeletonDemo />
            <Route path=path!("/card") view=skeleton_card::SkeletonCardDemo />
        </ParentRoute>
    }
    .into_inner()
}
