//! Leptos port of shadcn/ui Combobox component
//! 
//! Provides an autocomplete input component with a list of suggestions.

pub mod default;
pub mod new_york;

// Re-export common types
pub use default::{Combobox, ComboboxOption};

#[cfg(test)]
mod tests;
