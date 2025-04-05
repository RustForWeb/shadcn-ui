#[allow(clippy::module_inception)]
mod badge;
mod badge_destructive;
mod badge_outline;
mod badge_secondary;

use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes,
    components::{Outlet, ParentRoute, Route},
    path,
};

#[component(transparent)]
pub fn BadgeRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/badge") view=Outlet>
            <Route path=path!("/") view=badge::BadgeDemo />
            <Route path=path!("/destructive") view=badge_destructive::BadgeDestructive />
            <Route path=path!("/outline") view=badge_outline::BadgeOutline />
            <Route path=path!("/secondary") view=badge_secondary::BadgeSecondary />
        </ParentRoute>
    }
    .into_inner()
}
