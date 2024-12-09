use leptos::prelude::*;

use crate::default::components::ui::alert::{
    Alert,
    AlertTitle,
    AlertDescription,
};

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
