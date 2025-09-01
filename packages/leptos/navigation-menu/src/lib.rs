//! Leptos port of shadcn/ui navigation-menu

pub mod default;
pub mod new_york;

pub use default::{NavigationMenu};
pub use new_york::{NavigationMenu as NavigationMenuNewYork};

#[cfg(test)]
mod tests;
