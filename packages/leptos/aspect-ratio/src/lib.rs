//! Leptos port of [shadcn/ui Aspect Ratio](https://ui.shadcn.com/docs/components/aspect-ratio).
//!
//! Displays content within a desired ratio.
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/components/aspect-ratio.html) for more documenation.

pub mod default;
pub mod new_york;

// Re-export the components for easy access
pub use default::*;

#[cfg(feature = "new_york")]
pub use new_york as aspect_ratio;

#[cfg(test)]
mod tests;