// In actual projects this module would contain the copied components, but this example uses the local workspace packages.
<<<<<<< HEAD
#[cfg(feature = "badge")]
pub use shadcn_ui_leptos_badge::new_york as badge;
=======

#[cfg(feature = "alert")]
pub use shadcn_ui_leptos_alert::new_york as alert;

>>>>>>> main
#[cfg(any(feature = "button", feature = "card"))]
pub use shadcn_ui_leptos_button::new_york as button;

#[cfg(feature = "card")]
pub use shadcn_ui_leptos_card::new_york as card;
