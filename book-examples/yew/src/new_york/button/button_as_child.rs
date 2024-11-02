use yew::prelude::*;

use crate::new_york::components::ui::button::{Button, ButtonChildProps};

#[function_component]
pub fn ButtonAsChild() -> Html {
    html! {
        <Button
            as_child={Callback::from(|ButtonChildProps {class, ..}| html! {
                <a class={class} href="#/login">{"Login"}</a>
            })}
        />
    }
}
