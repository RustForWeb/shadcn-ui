use radix_yew_icons::ExclamationTriangleIcon;
use yew::prelude::*;

use crate::new_york::components::ui::alert::{Alert, AlertDescription, AlertTitle, AlertVariant};

#[function_component]
pub fn AlertDestructive() -> Html {
    html! {
        <Alert variant={AlertVariant::Destructive}>
            <ExclamationTriangleIcon class="h-4 w-4" />
            <AlertTitle>{"Error"}</AlertTitle>
            <AlertDescription>
                {"Your session has expired. Please log in again."}
            </AlertDescription>
        </Alert>
    }
}
