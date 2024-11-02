use radix_yew_icons::EnvelopeOpenIcon;
use yew::prelude::*;

use crate::new_york::components::ui::button::Button;

#[function_component]
pub fn ButtonWithIcon() -> Html {
    html! {
        <Button>
            <EnvelopeOpenIcon />
            {"Login with Email"}
        </Button>
    }
}
