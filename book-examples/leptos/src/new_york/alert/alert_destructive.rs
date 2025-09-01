use leptos::prelude::*;

#[cfg(feature = "lucide-leptos")]
use lucide_leptos::CircleAlert;

#[cfg(not(feature = "lucide-leptos"))]
const CircleAlert: () = ();

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
