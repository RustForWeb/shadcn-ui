mod app;
mod components;

#[cfg(feature = "alert")]
mod alert;
#[cfg(feature = "alert-destructive")]
mod alert_destructive;
#[cfg(feature = "button")]
mod button;
#[cfg(feature = "button-as-child")]
mod button_as_child;
#[cfg(feature = "button-destructive")]
mod button_destructive;
#[cfg(feature = "button-ghost")]
mod button_ghost;
#[cfg(feature = "button-icon")]
mod button_icon;
#[cfg(feature = "button-link")]
mod button_link;
#[cfg(feature = "button-loading")]
mod button_loading;
#[cfg(feature = "button-outline")]
mod button_outline;
#[cfg(feature = "button-secondary")]
mod button_secondary;
#[cfg(feature = "button-with-icon")]
mod button_with_icon;

// #[cfg(feature = "card")]
// mod card;
// #[cfg(feature = "input")]
// mod input;
// #[cfg(feature = "textarea")]
// mod textarea;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
