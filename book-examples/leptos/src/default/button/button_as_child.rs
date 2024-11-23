use leptos::prelude::*;

use crate::default::components::ui::button::{Button, ButtonChildProps};

#[component]
pub fn ButtonAsChild() -> impl IntoView {
    view! {
        <Button
            as_child={Callback::new(|ButtonChildProps {class, ..}| {
                view! {
                    <a class={class} href="#/login">"Login"</a>
                }
                .into_any()
            })}
        />
    }
}
