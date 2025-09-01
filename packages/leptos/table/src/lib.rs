//! Leptos port of shadcn/ui table

pub mod default;
pub mod new_york;

pub use default::{Table};
pub use new_york::{Table as TableNewYork};

#[cfg(test)]
mod tests;
