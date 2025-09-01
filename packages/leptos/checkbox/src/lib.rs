//! Leptos port of shadcn/ui checkbox

pub mod default;
pub mod new_york;

pub use default::{Checkbox};
pub use new_york::{Checkbox as CheckboxNewYork};

#[cfg(test)]
mod tests;
