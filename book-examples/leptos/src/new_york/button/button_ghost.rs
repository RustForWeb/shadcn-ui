use leptos::prelude::*;

use crate::new_york::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn ButtonGhost() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Ghost}>"Ghost"</Button>
    }
}
