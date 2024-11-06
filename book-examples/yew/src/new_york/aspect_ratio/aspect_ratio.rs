use yew::prelude::*;

use crate::new_york::components::ui::aspect_ratio::AspectRatio;

#[function_component]
pub fn AspectRatioDemo() -> Html {
    html! {
        <AspectRatio ratio={16.0 / 9.0} class="bg-muted">
            <img
                src="https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800&dpr=2&q=80"
                alt="Photo by Drew Beamer"
                class="h-full w-full rounded-md object-cover"
            />
        </AspectRatio>
    }
}
