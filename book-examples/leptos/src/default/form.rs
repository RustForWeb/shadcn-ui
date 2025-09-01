mod form_example;

use leptos::prelude::*;
use leptos_router::{MatchNestedRoutes, path, components::Route};

#[component(transparent)]
pub fn FormRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <Route path=path!("form") view=form_example::FormExample />
    }
    .into_inner()
}
