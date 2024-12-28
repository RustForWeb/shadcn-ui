#[allow(clippy::module_inception)]
mod textarea;
mod textarea_disabled;
mod textarea_form;
mod textarea_with_button;
mod textarea_with_label;
mod textarea_with_text;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn TextareaRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/textarea") view=Outlet>
            <Route path=path!("/") view=textarea::TextareaDemo />
            <Route path=path!("/disabled") view=textarea_disabled::TextareaDisabledDemo />
            <Route path=path!("/form") view=textarea_form::TextareaFormDemo />
            <Route path=path!("/with-button") view=textarea_with_button::TextareaWithButtonDemo />
            <Route path=path!("/with-label") view=textarea_with_label::TextareaWithLabelDemo />
            <Route path=path!("/with-text") view=textarea_with_text::TextareaWithTextDemo />
        </ParentRoute>
    }
    .into_inner()
}
