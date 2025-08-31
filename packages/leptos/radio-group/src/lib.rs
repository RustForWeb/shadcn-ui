//! Leptos port of [shadcn/ui Radio Group](https://ui.shadcn.com/docs/components/radio-group).
//!
//! A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time.
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/components/radio-group.html) for more documenation.

pub mod default;
pub mod new_york;

// Re-export the components for easy access
pub use default::{RadioGroup, RadioGroupItem};
pub use new_york::{RadioGroup as RadioGroupNewYork, RadioGroupItem as RadioGroupItemNewYork};

#[cfg(test)]
mod tests;
