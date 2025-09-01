mod combobox_example;

use leptos::prelude::*;
use leptos_router::{MatchNestedRoutes, path, components::Route};

#[component(transparent)]
pub fn ComboboxRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <Route path=path!("combobox") view=combobox_example::ComboboxExample />
    }
    .into_inner()
}
