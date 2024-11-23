use leptos::prelude::*;
// use leptos_lucide_icons::ChevronRight;

use crate::default::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn ButtonIcon() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Outline} size={ButtonSize::Icon}>
            // TODO
            // <ChevronRight class="h-4 w-4" />
        </Button>
    }
}
