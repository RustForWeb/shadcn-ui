// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

#[cfg(feature = "alert")]
pub use shadcn_ui_yew_alert::default as alert;
#[cfg(feature = "aspect-ratio")]
pub use shadcn_ui_yew_aspect_ratio::default as aspect_ratio;
#[cfg(feature = "avatar")]
pub use shadcn_ui_yew_avatar::default as avatar;
#[cfg(feature = "badge")]
pub use shadcn_ui_yew_badge::default as badge;
#[cfg(any(feature = "button", feature = "card", feature = "input"))]
pub use shadcn_ui_yew_button::default as button;
#[cfg(feature = "card")]
pub use shadcn_ui_yew_card::default as card;
#[cfg(any(feature = "input", feature = "card"))]
pub use shadcn_ui_yew_input::default as input;
#[cfg(any(feature = "label", feature = "card", feature = "input"))]
pub use shadcn_ui_yew_label::default as label;
#[cfg(feature = "pagination")]
pub use shadcn_ui_yew_pagination::default as pagination;
#[cfg(feature = "separator")]
pub use shadcn_ui_yew_separator::default as separator;
#[cfg(feature = "skeleton")]
pub use shadcn_ui_yew_skeleton::default as skeleton;
