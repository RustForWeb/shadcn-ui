use leptos::prelude::*;
use lucide_leptos::ChevronRight;

use crate::default::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn ButtonIcon() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Outline} size={ButtonSize::Icon}>
            <ChevronRight attr:class="h-4 w-4" />
        </Button>
    }
}
