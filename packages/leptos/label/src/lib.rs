//! Leptos port of shadcn/ui label

pub mod default;
pub mod new_york;

pub use default::{Label};
pub use new_york::{Label as LabelNewYork};

#[cfg(test)]
mod tests;
