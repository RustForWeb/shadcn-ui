mod components;

#[cfg(feature = "button")]
mod button;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn NewYork() -> impl MatchNestedRoutes + Clone {
    let children = (
        #[cfg(feature = "button")]
        {
            component_view(self::button::ButtonRoutes, ())
        },
    );

    view! {
        <ParentRoute path=path!("new-york") view=Outlet children=ToChildren::to_children(move || children) />
    }
    .into_inner()
}
