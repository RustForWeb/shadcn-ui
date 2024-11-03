use yew::prelude::*;

use crate::new_york::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};

#[function_component]
pub fn AvatarDemo() -> Html {
    html! {
        <Avatar>
            <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
            <AvatarFallback>{"CN"}</AvatarFallback>
        </Avatar>
    }
}
