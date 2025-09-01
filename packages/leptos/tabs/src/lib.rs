//! Leptos port of shadcn/ui tabs

pub mod default;
pub mod new_york;

pub use default::{
    Tabs, TabsList, TabsTrigger, TabsContent
};
pub use new_york::{
    Tabs as TabsNewYork, TabsList as TabsListNewYork, TabsTrigger as TabsTriggerNewYork, TabsContent as TabsContentNewYork
};

#[cfg(test)]
mod tests;
