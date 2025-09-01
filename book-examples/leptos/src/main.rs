mod app;
mod default;
mod new_york;
mod lazy_loading;
mod bundle_analyzer;
mod dynamic_loader;

use leptos::*;
use leptos::mount::mount_to_body;
use crate::app::App;

fn main() {
    mount_to_body(|| view! { <App /> })
}
