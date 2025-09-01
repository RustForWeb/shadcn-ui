// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

// #[cfg(feature = "alert")]
// pub use shadcn_ui_leptos_alert::default as alert;
// #[cfg(feature = "badge")]
// pub use shadcn_ui_leptos_badge::default as badge;
#[cfg(any(feature = "button", feature = "card"))]
pub use shadcn_ui_leptos_button::default as button;
#[cfg(feature = "card")]
pub use shadcn_ui_leptos_card::default as card;
// #[cfg(feature = "input")]
// pub use shadcn_ui_leptos_input::default as input;
// #[cfg(feature = "checkbox")]
// pub use shadcn_ui_leptos_checkbox::default as checkbox;
// #[cfg(feature = "select")]
// pub use shadcn_ui_leptos_select::default as select;
// #[cfg(feature = "dialog")]
// pub use shadcn_ui_leptos_dialog::default as dialog;
// #[cfg(feature = "tabs")]
// pub use shadcn_ui_leptos_tabs::default as tabs;
// #[cfg(feature = "radio-group")]
// pub use shadcn_ui_leptos_radio_group::default as radio_group;
