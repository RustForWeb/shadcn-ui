use yew::prelude::*;
use yew_lucide::Loader2;

use crate::components::ui::button::Button;

#[function_component]
pub fn ButtonLoading() -> Html {
    html! {
        <Button disabled=true>
            <Loader2 class="mr-2 h-4 w-4 animate-spin" />
            {"Please wait"}
        </Button>
    }
}
