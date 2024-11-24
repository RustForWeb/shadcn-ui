use lucide_yew::Terminal;
use yew::prelude::*;

use crate::default::components::ui::alert::{Alert, AlertDescription, AlertTitle};

#[function_component]
pub fn AlertDemo() -> Html {
    html! {
        <Alert>
            <Terminal class="h-4 w-4" />
            <AlertTitle>{"Heads up!"}</AlertTitle>
            <AlertDescription>
                {"You can add components to your app using the cli."}
            </AlertDescription>
        </Alert>
    }
}
