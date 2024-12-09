use leptos::prelude::*;

use crate::default::components::ui::alert::Alert;
use crate::default::components::ui::alert::AlertDescription;
use crate::default::components::ui::alert::AlertTitle;

#[component]
pub fn AlertDemo() -> impl IntoView {
    view! {
        <Alert>
            <AlertTitle>"Title"</AlertTitle>
            <AlertDescription>"Description"</AlertDescription>
        </Alert>
    }
}
