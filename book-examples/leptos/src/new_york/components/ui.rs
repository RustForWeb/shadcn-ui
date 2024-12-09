// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

#[cfg(feature = "button")]
pub use shadcn_ui_leptos_button::new_york as button;

#[cfg(feature = "alert")]
pub use shadcn_ui_leptos_alert::new_york as alert;
