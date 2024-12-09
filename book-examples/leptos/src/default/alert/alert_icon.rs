use leptos::prelude::*;
use lucide_leptos::ChevronRight;

use crate::default::components::ui::alert::{
    Alert, 
    AlertTitle,
    AlertDescription, 
};

#[component]
pub fn AlertIcon() -> impl IntoView {
    view! {
        <Alert>
            <ChevronRight attr:class="h-4 w-4" />
            <AlertTitle>"Heads up!"</AlertTitle>
            <AlertDescription>
                "You can add components to your app using the cli."
            </AlertDescription>
        </Alert>
    }
}
