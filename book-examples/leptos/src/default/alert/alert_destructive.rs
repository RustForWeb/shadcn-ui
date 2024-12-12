use leptos::prelude::*;

use crate::default::components::ui::alert::{Alert, AlertDescription, AlertTitle, AlertVariant};

#[component]
pub fn AlertDestructive() -> impl IntoView {
    view! {
        <Alert variant={AlertVariant::Destructive}>
            <AlertTitle>"Heads up!"</AlertTitle>
            <AlertDescription>
                "You can add components to your app using the cli."
            </AlertDescription>
        </Alert>
    }
}
