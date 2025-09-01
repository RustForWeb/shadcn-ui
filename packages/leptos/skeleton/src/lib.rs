//! Leptos port of shadcn/ui skeleton

pub mod default;
pub mod new_york;

pub use default::{
    Skeleton, SkeletonText, SkeletonAvatar, SkeletonCard, SkeletonVariant, SkeletonSize
};
pub use new_york::{
    Skeleton as SkeletonNewYork, SkeletonText as SkeletonTextNewYork, 
    SkeletonAvatar as SkeletonAvatarNewYork, SkeletonCard as SkeletonCardNewYork,
    SkeletonVariant as SkeletonVariantNewYork, SkeletonSize as SkeletonSizeNewYork
};

#[cfg(test)]
mod tests;
