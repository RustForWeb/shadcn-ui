//! Leptos port of shadcn/ui button

pub mod default;
pub mod new_york;

pub use default::{Button, ButtonVariant, ButtonSize, ButtonChildProps};
pub use new_york::{Button as ButtonNewYork, ButtonVariant as ButtonVariantNewYork, ButtonSize as ButtonSizeNewYork, ButtonChildProps as ButtonChildPropsNewYork};

#[cfg(test)]
mod tests;
