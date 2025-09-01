//! Leptos port of shadcn/ui alert

pub mod default;
pub mod new_york;

pub use default::{Alert, AlertTitle, AlertDescription, AlertVariant};
pub use new_york::{Alert as AlertNewYork, AlertTitle as AlertTitleNewYork, AlertDescription as AlertDescriptionNewYork, AlertVariant as AlertVariantNewYork};

#[cfg(test)]
mod tests;
