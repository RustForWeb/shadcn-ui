#[allow(clippy::module_inception)]
mod input;
mod input_disabled;
mod input_file;
mod input_form;
mod input_with_button;
mod input_with_label;
mod input_with_text;

use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn InputRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/input") view=Outlet>
            <Route path=path!("/") view=input::InputDemo />
            <Route path=path!("/disabled") view=input_disabled::InputDisabledDemo />
            <Route path=path!("/file") view=input_file::InputFileDemo />
            <Route path=path!("/form") view=input_form::InputFormDemo />
            <Route path=path!("/with-button") view=input_with_button::InputWithButtonDemo />
            <Route path=path!("/with-label") view=input_with_label::InputWithLabelDemo />
            <Route path=path!("/with-text") view=input_with_text::InputWithTextDemo />
        </ParentRoute>
    }
    .into_inner()
}
