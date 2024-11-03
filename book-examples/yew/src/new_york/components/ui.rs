// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

#[cfg(feature = "alert")]
pub use shadcn_ui_yew_alert::new_york as alert;
#[cfg(feature = "avatar")]
pub use shadcn_ui_yew_avatar::new_york as avatar;
#[cfg(feature = "badge")]
pub use shadcn_ui_yew_badge::new_york as badge;
#[cfg(any(feature = "button", feature = "card"))]
pub use shadcn_ui_yew_button::new_york as button;
#[cfg(feature = "card")]
pub use shadcn_ui_yew_card::new_york as card;
#[cfg(feature = "label")]
pub use shadcn_ui_yew_label::new_york as label;
#[cfg(feature = "pagination")]
pub use shadcn_ui_yew_pagination::new_york as pagination;
#[cfg(feature = "skeleton")]
pub use shadcn_ui_yew_skeleton::new_york as skeleton;
