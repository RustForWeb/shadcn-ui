use leptos::prelude::*;

use crate::default::components::ui::button::Button;

#[component]
pub fn ButtonWithIcon() -> impl IntoView {
    view! {
        <Button>
            // TODO
            // <Mail class="mr-2 h-4 w-4" />
            "Login with Email"
        </Button>
    }
}
