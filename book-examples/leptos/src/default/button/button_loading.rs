use leptos::prelude::*;
use lucide_leptos::LoaderCircle;

use crate::default::components::ui::button::Button;

#[component]
pub fn ButtonLoading() -> impl IntoView {
    view! {
        <Button disabled=true>
            <LoaderCircle attr:class="mr-2 h-4 w-4 animate-spin" />
            "Please wait"
        </Button>
    }
}
