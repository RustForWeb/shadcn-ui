// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

#[cfg(any(feature = "button", feature = "card"))]
pub use shadcn_ui_leptos_button::new_york as button;
#[cfg(feature = "card")]
pub use shadcn_ui_leptos_card::new_york as card;
