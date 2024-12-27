// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

#[cfg(feature = "alert")]
pub use shadcn_ui_leptos_alert::new_york as alert;

#[cfg(feature = "badge")]
pub use shadcn_ui_leptos_badge::new_york as badge;

#[cfg(feature = "breadcrumb")]
pub use shadcn_ui_leptos_breadcrumb::new_york as breadcrumb;

#[cfg(any(feature = "button", feature = "card"))]
pub use shadcn_ui_leptos_button::new_york as button;

#[cfg(feature = "card")]
pub use shadcn_ui_leptos_card::new_york as card;

#[cfg(feature = "pagination")]
pub use shadcn_ui_leptos_pagination::new_york as pagination;
