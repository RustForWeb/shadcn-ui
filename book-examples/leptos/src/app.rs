use leptos::prelude::*;
use leptos_router::components::{Router, Routes};

use crate::{default::Default, new_york::NewYork};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex min-h-[350px] w-full justify-center p-10 items-center">
            // TODO: HashRouter
            <Router>
                <Routes fallback=|| "Not found">
                    <Default />
                    <NewYork />
                </Routes>
            </Router>
        </div>
    }
}
