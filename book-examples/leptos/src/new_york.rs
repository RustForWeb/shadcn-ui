mod components;

#[cfg(feature = "alert")]
mod alert;

#[cfg(feature = "button")]
mod button;

#[cfg(feature = "card")]
mod card;
#[cfg(feature = "input")]
mod input;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn NewYork() -> impl MatchNestedRoutes + Clone {
    let children = (
        #[cfg(feature = "alert")]
        {
            component_view(self::alert::AlertRoutes, ())
        },
        #[cfg(feature = "button")]
        {
            component_view(self::button::ButtonRoutes, ())
        },
        #[cfg(feature = "card")]
        {
            component_view(self::card::CardRoutes, ())
        },
        #[cfg(feature = "input")]
        {
            component_view(self::input::InputRoutes, ())
        },
    );

    view! {
        <ParentRoute path=path!("new-york") view=Outlet children=ToChildren::to_children(move || children) />
    }
    .into_inner()
}
