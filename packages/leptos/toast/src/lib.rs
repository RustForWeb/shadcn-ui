//! Leptos port of shadcn/ui toast

pub mod default;
pub mod new_york;

pub use default::{Toast};
pub use new_york::{Toast as ToastNewYork};

#[cfg(test)]
mod tests;
