//! Default utilities for Tailwind CSS classes.

use tailwind_fuse::tw_merge;

/// Combines multiple class names and merges Tailwind CSS classes.
/// This is equivalent to the `cn` helper function from shadcn/ui.
/// 
/// # Arguments
/// 
/// * `classes` - A slice of class name strings to combine
/// 
/// # Returns
/// 
/// A merged class string with Tailwind CSS classes properly handled
/// 
/// # Example
/// 
/// ```rust
/// use shadcn_ui_leptos_utils::cn;
/// 
/// let result = cn(&["bg-blue-500", "text-white", "p-4"]);
/// assert_eq!(result, "bg-blue-500 text-white p-4");
/// 
/// // Tailwind CSS classes are properly merged
/// let result = cn(&["p-2", "p-4"]); // p-4 will override p-2
/// assert_eq!(result, "p-4");
/// ```
pub fn cn(classes: &[&str]) -> String {
    tw_merge!(classes.join(" "))
}

/// Combines multiple class names and merges Tailwind CSS classes.
/// This is a more flexible version that accepts any type that can be converted to a string.
/// 
/// # Arguments
/// 
/// * `classes` - A slice of items that can be converted to strings
/// 
/// # Returns
/// 
/// A merged class string with Tailwind CSS classes properly handled
/// 
/// # Example
/// 
/// ```rust
/// use shadcn_ui_leptos_utils::cn_flexible;
/// 
/// let result = cn_flexible(&["bg-blue-500", "text-white", "p-4"]);
/// assert_eq!(result, "bg-blue-500 text-white p-4");
/// 
/// let result = cn_flexible(&[String::from("bg-blue-500"), "text-white".to_string()]);
/// assert_eq!(result, "bg-blue-500 text-white");
/// ```
pub fn cn_flexible<T: AsRef<str>>(classes: &[T]) -> String {
    let class_strings: Vec<&str> = classes.iter().map(|c| c.as_ref()).collect();
    tw_merge!(class_strings.join(" "))
}
