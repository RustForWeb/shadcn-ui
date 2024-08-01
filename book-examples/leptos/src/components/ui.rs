// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

#[cfg(any(feature = "alert", feature = "alert-destructive"))]
pub use shadcn_ui_leptos_alert::default as alert;

#[cfg(any(
    feature = "button",
    feature = "button-secondary",
    feature = "button-destructive",
    feature = "button-outline",
    feature = "button-ghost",
    feature = "button-link",
    feature = "button-icon",
    feature = "button-with-icon",
    feature = "button-loading",
    feature = "button-as-child"
))]
pub use shadcn_ui_leptos_button::default as button;
