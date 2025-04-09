mod components;

#[cfg(feature = "alert")]
mod alert;
#[cfg(feature = "badge")]
mod badge;
#[cfg(feature = "button")]
mod button;
#[cfg(feature = "card")]
mod card;
#[cfg(feature = "skeleton")]
mod skeleton;

use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes,
    components::{Outlet, ParentRoute},
    path,
};

#[component(transparent)]
pub fn Default() -> impl MatchNestedRoutes + Clone {
    let children = (
        #[cfg(feature = "alert")]
        {
            component_view(self::alert::AlertRoutes, ())
        },
        #[cfg(feature = "badge")]
        {
            component_view(self::badge::BadgeRoutes, ())
        },
        #[cfg(feature = "button")]
        {
            component_view(self::button::ButtonRoutes, ())
        },
        #[cfg(feature = "card")]
        {
            component_view(self::card::CardRoutes, ())
        },
        #[cfg(feature = "skeleton")]
        {
            component_view(self::skeleton::SkeletonRoutes, ())
        },
    );

    view! {
        <ParentRoute path=path!("default") view=Outlet children=ToChildren::to_children(move || children) />
    }
    .into_inner()
}
