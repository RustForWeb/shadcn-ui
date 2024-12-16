use leptos::prelude::*;
use lucide_leptos::Terminal;

use crate::default::components::ui::alert::{Alert, AlertDescription, AlertTitle};

#[component]
pub fn AlertDemo() -> impl IntoView {
    view! {
        <Alert>
            <Terminal attr:class="h-4 w-4" />
            <AlertTitle>"Heads up!"</AlertTitle>
            <AlertDescription>
                "You can add components to your app using the cli."
            </AlertDescription>
        </Alert>
    }
}
