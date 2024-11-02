use radix_yew_icons::ChevronRightIcon;
use yew::prelude::*;

use crate::new_york::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[function_component]
pub fn ButtonIcon() -> Html {
    html! {
        <Button variant={ButtonVariant::Outline} size={ButtonSize::Icon}>
            <ChevronRightIcon class="h-4 w-4" />
        </Button>
    }
}
