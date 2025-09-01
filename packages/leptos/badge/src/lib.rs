//! Leptos port of shadcn/ui badge

pub mod default;
pub mod new_york;

pub use default::{Badge, BadgeVariant};
pub use new_york::{Badge as BadgeNewYork, BadgeVariant as BadgeVariantNewYork};

#[cfg(test)]
mod tests;
