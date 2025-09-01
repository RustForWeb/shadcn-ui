//! Leptos port of shadcn/ui card

pub mod default;
pub mod new_york;

pub use default::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
pub use new_york::{Card as CardNewYork, CardHeader as CardHeaderNewYork, CardTitle as CardTitleNewYork, CardDescription as CardDescriptionNewYork, CardContent as CardContentNewYork, CardFooter as CardFooterNewYork};

#[cfg(test)]
mod tests;
