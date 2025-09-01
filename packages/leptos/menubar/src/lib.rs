//! Leptos port of shadcn/ui menubar

pub mod default;
pub mod new_york;

pub use default::{Menubar};
pub use new_york::{Menubar as MenubarNewYork};

#[cfg(test)]
mod tests;
