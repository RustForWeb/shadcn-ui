// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

#[cfg(feature = "alert")]
pub use shadcn_ui_leptos_alert::default as alert;

#[cfg(any(feature = "button", feature = "card"))]
pub use shadcn_ui_leptos_button::default as button;

#[cfg(feature = "card")]
pub use shadcn_ui_leptos_card::default as card;
#[cfg(feature = "input")]
pub use shadcn_ui_leptos_input::default as input;
