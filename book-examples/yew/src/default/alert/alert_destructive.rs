use yew::prelude::*;
use yew_lucide::AlertCircle;

use crate::default::components::ui::alert::{Alert, AlertDescription, AlertTitle, AlertVariant};

#[function_component]
pub fn AlertDestructive() -> Html {
    html! {
        <Alert variant={AlertVariant::Destructive}>
            <AlertCircle class="h-4 w-4" />
            <AlertTitle>{"Error"}</AlertTitle>
            <AlertDescription>
                {"Your session has expired. Please log in again."}
            </AlertDescription>
        </Alert>
    }
}
