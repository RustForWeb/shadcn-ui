use leptos::prelude::*;
// use radix_leptos_icons::ReloadIcon;

use crate::new_york::components::ui::button::Button;

#[component]
pub fn ButtonLoading() -> impl IntoView {
    view! {
        <Button disabled=true>
            // TODO
            // <ReloadIcon class="mr-2 h-4 w-4 animate-spin" />
            "Please wait"
        </Button>
    }
}
