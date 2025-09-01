use leptos::prelude::*;

#[cfg(feature = "lucide-leptos")]
use lucide_leptos::LoaderCircle;

#[cfg(not(feature = "lucide-leptos"))]
const LoaderCircle: () = ();

use crate::default::components::ui::button::Button;

#[component]
pub fn ButtonLoading() -> impl IntoView {
    view! {
        <Button disabled=true>
            {
                #[cfg(feature = "lucide-leptos")]
                {
                    view! { <LoaderCircle attr:class="mr-2 h-4 w-4 animate-spin" /> }
                }
                #[cfg(not(feature = "lucide-leptos"))]
                {
                    view! { <span class="mr-2 h-4 w-4 animate-spin">"⏳"</span> }
                }
            }
            "Please wait"
        </Button>
    }
}
