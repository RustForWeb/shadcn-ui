use leptos::prelude::*;

use crate::default::components::ui::{button::Button, textarea::Textarea};

#[component]
pub fn TextareaWithButtonDemo() -> impl IntoView {
    let text = RwSignal::new("".to_string());
    view! {
        <div class="grid w-full gap-2">
            <Textarea placeholder="Type your message here." value=text />
            <Button
                onclick= Callback::new(move |_|{
                    text.set("Updated".to_string());
                })
            >"Send message"</Button>
        </div>
    }
}
