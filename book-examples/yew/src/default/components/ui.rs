// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

#[cfg(feature = "alert")]
pub use shadcn_ui_yew_alert::default as alert;
#[cfg(feature = "avatar")]
pub use shadcn_ui_yew_avatar::default as avatar;
#[cfg(feature = "badge")]
pub use shadcn_ui_yew_badge::default as badge;
#[cfg(any(feature = "button", feature = "card"))]
pub use shadcn_ui_yew_button::default as button;
#[cfg(feature = "card")]
pub use shadcn_ui_yew_card::default as card;
#[cfg(feature = "label")]
pub use shadcn_ui_yew_label::default as label;
#[cfg(feature = "pagination")]
pub use shadcn_ui_yew_pagination::default as pagination;
#[cfg(feature = "skeleton")]
pub use shadcn_ui_yew_skeleton::default as skeleton;
