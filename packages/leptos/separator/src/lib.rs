//! Leptos port of shadcn/ui separator

pub mod default;
pub mod new_york;

pub use default::{Separator};
pub use new_york::{Separator as SeparatorNewYork};

#[cfg(test)]
mod tests;
