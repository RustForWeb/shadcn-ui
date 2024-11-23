use leptos::prelude::*;

use crate::default::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn ButtonGhost() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Ghost}>"Ghost"</Button>
    }
}
