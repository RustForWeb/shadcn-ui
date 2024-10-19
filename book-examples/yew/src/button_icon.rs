use yew::prelude::*;
use yew_lucide::ChevronRight;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[function_component]
pub fn ButtonIcon() -> Html {
    html! {
        <Button variant={ButtonVariant::Outline} size={ButtonSize::Icon}>
            <ChevronRight class="h-4 w-4" />
        </Button>
    }
}
