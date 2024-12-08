mod components;

#[cfg(feature = "button")]
mod button;
#[cfg(feature = "card")]
mod card;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn Default() -> impl MatchNestedRoutes + Clone {
    let children = (
        #[cfg(feature = "button")]
        {
            component_view(self::button::ButtonRoutes, ())
        },
        #[cfg(feature = "card")]
        {
            component_view(self::card::CardRoutes, ())
        },
    );

    view! {
        <ParentRoute path=path!("default") view=Outlet children=ToChildren::to_children(move || children) />
    }
    .into_inner()
}
