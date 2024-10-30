use leptos::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn ButtonLoading() -> impl IntoView {
    view! {
        <Button attr:disabled=true>
            {/* <Loader2 className="mr-2 h-4 w-4 animate-spin" /> */}
            Please wait
        </Button>
    }
}
