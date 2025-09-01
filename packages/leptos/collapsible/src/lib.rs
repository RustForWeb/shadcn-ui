//! Leptos port of shadcn/ui collapsible

pub mod default;
pub mod new_york;

pub use default::{
    Collapsible, CollapsibleTrigger, CollapsibleContent,
};

pub use new_york::{
    Collapsible as CollapsibleNewYork,
    CollapsibleTrigger as CollapsibleTriggerNewYork,
    CollapsibleContent as CollapsibleContentNewYork,
};

#[cfg(test)]
mod tests;