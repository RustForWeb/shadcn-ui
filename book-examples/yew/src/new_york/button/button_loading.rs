use radix_yew_icons::ReloadIcon;
use yew::prelude::*;

use crate::new_york::components::ui::button::Button;

#[function_component]
pub fn ButtonLoading() -> Html {
    html! {
        <Button disabled=true>
            <ReloadIcon class="mr-2 h-4 w-4 animate-spin" />
            {"Please wait"}
        </Button>
    }
}
