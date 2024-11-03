use leptos::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn ButtonWithIcon() -> impl IntoView {
    view! {
        <Button>
            {/* <Mail class="mr-2 h-4 w-4" /> */}
            Login with Email
        </Button>
    }
}
