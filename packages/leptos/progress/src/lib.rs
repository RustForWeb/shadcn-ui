//! Leptos port of shadcn/ui progress

pub mod default;
pub mod new_york;

pub use default::{
    Progress, ProgressRoot, ProgressIndicator, ProgressLabel, ProgressVariant
};
pub use new_york::{
    Progress as ProgressNewYork, ProgressRoot as ProgressRootNewYork, 
    ProgressIndicator as ProgressIndicatorNewYork, ProgressLabel as ProgressLabelNewYork,
    ProgressVariant as ProgressVariantNewYork
};

#[cfg(test)]
mod tests;
