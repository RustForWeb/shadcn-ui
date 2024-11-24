use lucide_yew::LoaderCircle;
use yew::prelude::*;

use crate::default::components::ui::button::Button;

#[function_component]
pub fn ButtonLoading() -> Html {
    html! {
        <Button disabled=true>
            <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
            {"Please wait"}
        </Button>
    }
}
