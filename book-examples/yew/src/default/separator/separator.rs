use yew::prelude::*;

use crate::default::components::ui::separator::{Orientation, Separator};

#[function_component]
pub fn SeparatorDemo() -> Html {
    html! {
        <div>
            <div class="space-y-1">
                <h4 class="text-sm font-medium leading-none">{"Radix Primitives"}</h4>
                <p class="text-sm text-muted-foreground">
                    {"An open-source UI component library."}
                </p>
            </div>
            <Separator class="my-4" />
            <div class="flex h-5 items-center space-x-4 text-sm">
                <div>{"Blog"}</div>
                <Separator orientation={Orientation::Vertical} />
                <div>{"Docs"}</div>
                <Separator orientation={Orientation::Vertical} />
                <div>{"Source"}</div>
            </div>
        </div>
    }
}
