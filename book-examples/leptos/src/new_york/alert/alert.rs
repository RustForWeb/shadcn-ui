use leptos::prelude::*;

use crate::new_york::components::ui::alert::{Alert, AlertDescription, AlertTitle};

#[component]
pub fn AlertDemo() -> impl IntoView {
    view! {
        <Alert>
            <AlertTitle>"Heads up!"</AlertTitle>
            <AlertDescription>
                "You can add components to your app using the cli."
            </AlertDescription>
        </Alert>
    }
}
