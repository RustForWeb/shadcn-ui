mod components;

// #[cfg(feature = "alert")]
// mod alert;
// #[cfg(feature = "badge")]
// mod badge;
#[cfg(feature = "button")]
mod button;
#[cfg(feature = "card")]
mod card;
// #[cfg(feature = "radio-group")]
// mod radio_group;

use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes,
    components::{Outlet, ParentRoute},
    path,
};

#[component(transparent)]
pub fn NewYork() -> impl MatchNestedRoutes + Clone {
    let children = (
        // #[cfg(feature = "alert")]
        // {
        //     component_view(self::alert::AlertRoutes, ())
        // },
        // #[cfg(feature = "badge")]
        // {
        //     component_view(self::badge::BadgeRoutes, ())
        // },
        #[cfg(feature = "button")]
        {
            component_view(self::button::ButtonRoutes, ())
        },
        #[cfg(feature = "card")]
        {
            component_view(self::card::CardRoutes, ())
        },
        // #[cfg(feature = "radio-group")]
        // {
        //     component_view(self::radio_group::RadioGroupRoutes, ())
        // },
    );

    view! {
        <ParentRoute path=path!("new-york") view=Outlet children=ToChildren::to_children(move || children) />
    }
    .into_inner()
}
