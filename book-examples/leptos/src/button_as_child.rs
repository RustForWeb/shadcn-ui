use leptos::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn ButtonAsChild() -> impl IntoView {
    view! {
        <Button> // TODO: as_child=true
            <a href="/login">Login</a>
        </Button>
    }
}
