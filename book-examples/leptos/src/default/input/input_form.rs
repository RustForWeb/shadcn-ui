use leptos::prelude::*;

use crate::default::components::ui::input::{ Input ,  InputType};

#[component]
pub fn InputFormDemo() -> impl IntoView {
    view! {
        <h1>"Input Form needs to be implemented yet"</h1>
        <Input r#type=InputType::Password placeholder="Email" />
    }
}
