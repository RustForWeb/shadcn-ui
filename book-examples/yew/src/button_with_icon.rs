use yew::prelude::*;
use yew_lucide::Mail;

use crate::components::ui::button::Button;

#[function_component]
pub fn ButtonWithIcon() -> Html {
    html! {
        <Button>
            <Mail class="mr-2 h-4 w-4" />
            {"Login with Email"}
        </Button>
    }
}
