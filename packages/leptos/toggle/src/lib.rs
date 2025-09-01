//! Leptos port of shadcn/ui toggle

pub mod default;
pub mod new_york;

pub use default::{Toggle};
pub use new_york::{Toggle as ToggleNewYork};

#[cfg(test)]
mod tests;
