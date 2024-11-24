use lucide_yew::Mail;
use yew::prelude::*;

use crate::default::components::ui::button::Button;

#[function_component]
pub fn ButtonWithIcon() -> Html {
    html! {
        <Button>
            <Mail class="mr-2 h-4 w-4" />
            {"Login with Email"}
        </Button>
    }
}
