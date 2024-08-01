use leptos::*;

use crate::components::ui::alert::{Alert, AlertDescription, AlertTitle, AlertVariant};

#[component]
pub fn AlertDestructive() -> impl IntoView {
    view! {
        <Alert variant=AlertVariant::Destructive>
            // <AlertCircle class="h-4 w-4" />
            <AlertTitle>Error</AlertTitle>
            <AlertDescription>
                Your session has expired. Please log in again.
            </AlertDescription>
        </Alert>
    }
}
