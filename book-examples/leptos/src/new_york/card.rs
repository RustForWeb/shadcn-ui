#[allow(clippy::module_inception)]
mod card;
mod card_with_form;

use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes,
    components::{Outlet, ParentRoute, Route},
    path,
};

#[component(transparent)]
pub fn CardRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/card") view=Outlet>
            <Route path=path!("/") view=card::CardDemo />
            <Route path=path!("/with-form") view=card_with_form::CardWithForm />
        </ParentRoute>
    }
    .into_inner()
}
