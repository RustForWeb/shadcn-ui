// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

#[cfg(feature = "button")]
pub use shadcn_ui_yew_button::new_york as button;
#[cfg(feature = "skeleton")]
pub use shadcn_ui_yew_skeleton::new_york as skeleton;
