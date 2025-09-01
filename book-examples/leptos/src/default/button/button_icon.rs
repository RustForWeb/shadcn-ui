use leptos::prelude::*;

#[cfg(feature = "lucide-leptos")]
use lucide_leptos::ChevronRight;

#[cfg(not(feature = "lucide-leptos"))]
const ChevronRight: () = ();

use crate::default::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn ButtonIcon() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Outline} size={ButtonSize::Icon}>
            {
                #[cfg(feature = "lucide-leptos")]
                {
                    view! { <ChevronRight attr:class="h-4 w-4" /> }
                }
                #[cfg(not(feature = "lucide-leptos"))]
                {
                    view! { <span>"→"</span> }
                }
            }
        </Button>
    }
}
