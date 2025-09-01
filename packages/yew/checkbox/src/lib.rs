//! Yew port of [shadcn/ui Checkbox](https://ui.shadcn.com/docs/components/checkbox).
//!
//! A control that allows the user to toggle between checked and not checked.
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/components/checkbox.html) for more documenation.

pub mod default;
pub mod new_york;

// Re-export the components for easy access
pub use default::*;

// #[cfg(feature = "new_york")]
// pub use new_york as checkbox;