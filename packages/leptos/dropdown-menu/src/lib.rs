//! Leptos port of shadcn/ui dropdown-menu

pub mod default;
pub mod new_york;

pub use default::{DropdownMenu};
pub use new_york::{DropdownMenu as DropdownMenuNewYork};

#[cfg(test)]
mod tests;
