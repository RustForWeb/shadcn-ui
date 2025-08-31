//! Leptos port of [shadcn/ui Tooltip](https://ui.shadcn.com/docs/components/tooltip).
//!
//! A tooltip component for displaying additional information on hover or focus.
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/components/tooltip.html) for more documentation.

pub mod default;
pub mod new_york;

#[cfg(test)]
mod tests;

// Re-export the components for easy access
pub use default::*;

#[cfg(feature = "new_york")]
pub use new_york as tooltip;
