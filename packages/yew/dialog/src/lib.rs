//! Yew port of [shadcn/ui Dialog](https://ui.shadcn.com/docs/components/dialog).

pub mod default;
pub mod new_york;

// Re-export the components for easy access
pub use default::{Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger};
pub use new_york::{Dialog as DialogNewYork, DialogContent as DialogContentNewYork, DialogDescription as DialogDescriptionNewYork, DialogFooter as DialogFooterNewYork, DialogHeader as DialogHeaderNewYork, DialogTitle as DialogTitleNewYork, DialogTrigger as DialogTriggerNewYork};

#[cfg(test)]
mod tests;
