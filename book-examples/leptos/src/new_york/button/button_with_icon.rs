use leptos::prelude::*;
// use radix_leptos_icons::EnvelopeOpenIcon;

use crate::new_york::components::ui::button::Button;

#[component]
pub fn ButtonWithIcon() -> impl IntoView {
    view! {
        <Button>
            // TODO
            // <EnvelopeOpenIcon />
            "Login with Email"
        </Button>
    }
}
