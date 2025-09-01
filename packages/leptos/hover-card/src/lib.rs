//! Leptos port of shadcn/ui hover-card

pub mod default;
pub mod new_york;

pub use default::{HoverCard};
pub use new_york::{HoverCard as HoverCardNewYork};

#[cfg(test)]
mod tests;
