#[allow(clippy::module_inception)]
mod button;
mod button_as_child;
mod button_destructive;
mod button_ghost;
mod button_icon;
mod button_link;
mod button_loading;
mod button_outline;
mod button_secondary;
mod button_with_icon;

use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes,
    components::{Outlet, ParentRoute, Route},
    path,
};

#[component(transparent)]
pub fn ButtonRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/button") view=Outlet>
            <Route path=path!("/") view=button::ButtonDemo />
            <Route path=path!("/as-child") view=button_as_child::ButtonAsChild />
            <Route path=path!("/destructive") view=button_destructive::ButtonDestructive />
            <Route path=path!("/ghost") view=button_ghost::ButtonGhost />
            <Route path=path!("/icon") view=button_icon::ButtonIcon />
            <Route path=path!("/link") view=button_link::ButtonLink />
            <Route path=path!("/loading") view=button_loading::ButtonLoading />
            <Route path=path!("/outline") view=button_outline::ButtonOutline />
            <Route path=path!("/secondary") view=button_secondary::ButtonSecondary />
            <Route path=path!("/with-icon") view=button_with_icon::ButtonWithIcon />
        </ParentRoute>
    }
    .into_inner()
}
