//! Leptos port of shadcn/ui context menu

pub mod default;
pub mod new_york;

pub use default::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
    ContextMenuSeparator, ContextMenuLabel, ContextMenuCheckboxItem,
    ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSub,
    ContextMenuSubContent, ContextMenuSubTrigger, ContextMenuShortcut,
};

pub use new_york::{
    ContextMenu as ContextMenuNewYork,
    ContextMenuContent as ContextMenuContentNewYork,
    ContextMenuItem as ContextMenuItemNewYork,
    ContextMenuTrigger as ContextMenuTriggerNewYork,
    ContextMenuSeparator as ContextMenuSeparatorNewYork,
    ContextMenuLabel as ContextMenuLabelNewYork,
    ContextMenuCheckboxItem as ContextMenuCheckboxItemNewYork,
    ContextMenuRadioGroup as ContextMenuRadioGroupNewYork,
    ContextMenuRadioItem as ContextMenuRadioItemNewYork,
    ContextMenuSub as ContextMenuSubNewYork,
    ContextMenuSubContent as ContextMenuSubContentNewYork,
    ContextMenuSubTrigger as ContextMenuSubTriggerNewYork,
    ContextMenuShortcut as ContextMenuShortcutNewYork,
};

#[cfg(test)]
mod tests;