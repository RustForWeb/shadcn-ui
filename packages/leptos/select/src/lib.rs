//! Leptos port of [shadcn/ui Select](https://ui.shadcn.com/docs/components/select).
//!
//! Component description here.
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/components/select.html) for more documentation.

pub mod default;
pub mod new_york;

// Re-export the components for easy access
pub use default::*;

#[cfg(feature = "new_york")]
pub use new_york as select;

#[cfg(test)]
mod tests;
