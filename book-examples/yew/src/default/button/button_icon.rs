use lucide_yew::ChevronRight;
use yew::prelude::*;

use crate::default::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[function_component]
pub fn ButtonIcon() -> Html {
    html! {
        <Button variant={ButtonVariant::Outline} size={ButtonSize::Icon}>
            <ChevronRight class="h-4 w-4" />
        </Button>
    }
}
