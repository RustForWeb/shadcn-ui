mod components;

#[cfg(feature = "button")]
mod button;
//#[cfg(feature = "alert")]
//mod alert;

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
      //  #[cfg(feature = "alert")]
      //  {
      //      component_view(self::alert::AlertRoutes, ())
      //  },
    );

    view! {
        <ParentRoute path=path!("default") view=Outlet children=ToChildren::to_children(move || children) />
    }
    .into_inner()
}
