use leptos::prelude::*;

use crate::new_york::components::ui::button::{Button};

#[component]
pub fn ButtonAsChild() -> impl IntoView {
    view! {
        <Button>
            <a href="#/login">"Login"</a>
        </Button>
    }
}
