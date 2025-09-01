//! Leptos port of shadcn/ui drawer

pub mod default;
pub mod new_york;

pub use default::{
    Drawer, DrawerTrigger, DrawerContent, DrawerHeader, DrawerFooter,
    DrawerTitle, DrawerDescription, DrawerClose, DrawerOverlay, DrawerPortal,
    DrawerNestedRoot, DrawerDirection,
};

pub use new_york::{
    Drawer as DrawerNewYork,
    DrawerTrigger as DrawerTriggerNewYork,
    DrawerContent as DrawerContentNewYork,
    DrawerHeader as DrawerHeaderNewYork,
    DrawerFooter as DrawerFooterNewYork,
    DrawerTitle as DrawerTitleNewYork,
    DrawerDescription as DrawerDescriptionNewYork,
    DrawerClose as DrawerCloseNewYork,
    DrawerOverlay as DrawerOverlayNewYork,
    DrawerPortal as DrawerPortalNewYork,
    DrawerNestedRoot as DrawerNestedRootNewYork,
    DrawerDirection as DrawerDirectionNewYork,
};

#[cfg(test)]
mod tests;