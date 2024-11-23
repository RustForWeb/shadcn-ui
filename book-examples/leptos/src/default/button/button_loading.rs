use leptos::prelude::*;
// use leptos_lucide_icons::Loader2;

use crate::default::components::ui::button::Button;

#[component]
pub fn ButtonLoading() -> impl IntoView {
    view! {
        <Button disabled=true>
            // TODO
            // <Loader2 class="mr-2 h-4 w-4 animate-spin" />
            "Please wait"
        </Button>
    }
}
