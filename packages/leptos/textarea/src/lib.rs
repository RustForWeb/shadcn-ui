//! Leptos port of shadcn/ui textarea

pub mod default;
pub mod new_york;

pub use default::{Textarea};
pub use new_york::{Textarea as TextareaNewYork};

#[cfg(test)]
mod tests;
