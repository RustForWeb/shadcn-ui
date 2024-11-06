// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

#[cfg(feature = "alert")]
pub use shadcn_ui_yew_alert::new_york as alert;
#[cfg(feature = "aspect-ratio")]
pub use shadcn_ui_yew_aspect_ratio::new_york as aspect_ratio;
#[cfg(feature = "avatar")]
pub use shadcn_ui_yew_avatar::new_york as avatar;
#[cfg(feature = "badge")]
pub use shadcn_ui_yew_badge::new_york as badge;
#[cfg(any(feature = "button", feature = "card", feature = "input"))]
pub use shadcn_ui_yew_button::new_york as button;
#[cfg(feature = "card")]
pub use shadcn_ui_yew_card::new_york as card;
#[cfg(any(feature = "input", feature = "card"))]
pub use shadcn_ui_yew_input::new_york as input;
#[cfg(any(feature = "label", feature = "card", feature = "input"))]
pub use shadcn_ui_yew_label::new_york as label;
#[cfg(feature = "pagination")]
pub use shadcn_ui_yew_pagination::new_york as pagination;
#[cfg(feature = "separator")]
pub use shadcn_ui_yew_separator::new_york as separator;
#[cfg(feature = "skeleton")]
pub use shadcn_ui_yew_skeleton::new_york as skeleton;
