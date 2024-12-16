use leptos::prelude::*;
use lucide_leptos::CircleAlert;

use crate::new_york::components::ui::alert::{Alert, AlertDescription, AlertTitle, AlertVariant};

#[component]
pub fn AlertDestructive() -> impl IntoView {
    view! {
        <Alert variant={AlertVariant::Destructive}>
            <CircleAlert attr:class="h-4 w-4" />
            <AlertTitle>"Error"</AlertTitle>
            <AlertDescription>
                "Your session has expired. Please log in again."
            </AlertDescription>
        </Alert>
    }
}
