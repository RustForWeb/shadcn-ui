//! Leptos port of [shadcn/ui utils](https://ui.shadcn.com/docs/installation/manual#add-a-cn-helper).
//!
//! Utility for Tailwind CSS classes.
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/) for more documentation.

pub mod default;
pub mod new_york;

// Re-export the main utility functions for convenience
pub use default::{cn, cn_flexible};

#[cfg(test)]
mod tests;
