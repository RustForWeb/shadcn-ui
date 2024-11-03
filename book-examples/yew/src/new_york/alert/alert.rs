use radix_yew_icons::RocketIcon;
use yew::prelude::*;

use crate::new_york::components::ui::alert::{Alert, AlertDescription, AlertTitle};

#[function_component]
pub fn AlertDemo() -> Html {
    html! {
        <Alert>
            <RocketIcon class="h-4 w-4" />
            <AlertTitle>{"Heads up!"}</AlertTitle>
            <AlertDescription>
                {"You can add components to your app using the cli."}
            </AlertDescription>
        </Alert>
    }
}
