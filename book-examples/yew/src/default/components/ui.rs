// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

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
pub use shadcn_ui_yew_button::default as button;
#[cfg(any(feature = "skeleton", feature = "skeleton-card",))]
pub use shadcn_ui_yew_skeleton::default as skeleton;
