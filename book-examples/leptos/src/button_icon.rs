use leptos::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn ButtonIcon() -> impl IntoView {
    view! {
        <Button variant=ButtonVariant::Outline size=ButtonSize::Icon>
            {/* <ChevronRight className="h-4 w-4" /> */}
        </Button>
    }
}
