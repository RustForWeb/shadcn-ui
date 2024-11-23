use leptos::prelude::*;
// use radix_leptos_icons::ChevronRightIcon;

use crate::new_york::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn ButtonIcon() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Outline} size={ButtonSize::Icon}>
            // TODO
            // <ChevronRightIcon class="h-4 w-4" />
        </Button>
    }
}
