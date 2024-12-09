use leptos::prelude::*;

use crate::default::components::ui::alert::{
    Alert, 
    AlertDescription, 
    AlertTitle, 
    AlertVariant
};


#[component]
pub fn AlertDestructive() -> impl IntoView {
    view! {
        <Alert variant={AlertVariant::Destructive}>
            <AlertTitle>"Title"</AlertTitle>
            <AlertDescription>"Description"</AlertDescription>
        </Alert>
    }
}
