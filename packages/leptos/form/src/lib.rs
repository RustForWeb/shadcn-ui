//! Leptos port of shadcn/ui Form component
//! 
//! Provides form building blocks with validation and accessibility features.

pub mod default;
pub mod new_york;

// Re-export common types
pub use default::{Form, FormField, FormItem, FormLabel, FormControl, FormMessage, FormDescription};

#[cfg(test)]
mod tests;
