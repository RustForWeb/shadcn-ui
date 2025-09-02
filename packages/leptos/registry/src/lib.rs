//! Feature-based component registry for shadcn/ui Leptos
//! 
//! This module provides conditional compilation of components based on features,
//! enabling code splitting and reducing bundle size by only including used components.

/// Component registry that conditionally includes components based on features
pub struct ComponentRegistry;

impl ComponentRegistry {
    /// Get all available component features
    pub fn available_features() -> Vec<&'static str> {
        vec![
            "alert",
            "badge", 
            "button",
            "card",
            "checkbox",
            "combobox",
            "dialog",
            "form",
            "input",
            "label",
            "pagination",
            "radio-group",
            "select",
            "separator",
            "skeleton",
            "switch",
            "table",
            "tabs",
            "textarea",
            "tooltip",
            "utils",
        ]
    }

    /// Check if a component feature is enabled
    pub fn has_feature(feature: &str) -> bool {
        // Note: This is a runtime check, not a compile-time feature gate
        // For compile-time feature gates, use cfg!(feature = "feature_name")
        match feature {
            "alert" => cfg!(feature = "alert"),
            "badge" => cfg!(feature = "badge"),
            "button" => cfg!(feature = "button"),
            "card" => cfg!(feature = "card"),
            "checkbox" => cfg!(feature = "checkbox"),
            "combobox" => cfg!(feature = "combobox"),
            "dialog" => cfg!(feature = "dialog"),
            "form" => cfg!(feature = "form"),
            "input" => cfg!(feature = "input"),
            "label" => cfg!(feature = "label"),
            "pagination" => cfg!(feature = "pagination"),
            "radio-group" => cfg!(feature = "radio-group"),
            "select" => cfg!(feature = "select"),
            "separator" => cfg!(feature = "separator"),
            "skeleton" => cfg!(feature = "skeleton"),
            "switch" => cfg!(feature = "switch"),
            "table" => cfg!(feature = "table"),
            "tabs" => cfg!(feature = "tabs"),
            "textarea" => cfg!(feature = "textarea"),
            "tooltip" => cfg!(feature = "tooltip"),
            "utils" => cfg!(feature = "utils"),
            _ => false,
        }
    }

    /// Get enabled component features
    pub fn enabled_features() -> Vec<&'static str> {
        Self::available_features()
            .into_iter()
            .filter(|&f| Self::has_feature(f))
            .collect()
    }

    /// Get bundle size estimate for enabled features
    pub fn bundle_size_estimate() -> usize {
        let enabled = Self::enabled_features();
        let mut total_size = 0;
        
        for feature in enabled {
            total_size += Self::feature_size_estimate(feature);
        }
        
        total_size
    }

    /// Estimate size for each component feature (in bytes)
    fn feature_size_estimate(feature: &str) -> usize {
        match feature {
            "alert" => 15_000,
            "badge" => 8_000,
            "button" => 12_000,
            "card" => 18_000,
            "checkbox" => 20_000,
            "combobox" => 35_000,
            "dialog" => 45_000,
            "form" => 50_000,
            "input" => 15_000,
            "label" => 5_000,
            "pagination" => 25_000,
            "radio-group" => 22_000,
            "select" => 30_000,
            "separator" => 3_000,
            "skeleton" => 8_000,
            "switch" => 18_000,
            "table" => 40_000,
            "tabs" => 28_000,
            "textarea" => 12_000,
            "tooltip" => 20_000,
            "utils" => 10_000,
            _ => 0,
        }
    }

    /// Get optimization recommendations based on current features
    pub fn optimization_recommendations() -> Vec<String> {
        let enabled = Self::enabled_features();
        let total_size = Self::bundle_size_estimate();
        let mut recommendations = Vec::new();

        if total_size > 2_000_000 { // 2MB
            recommendations.push("Bundle size is large (>2MB). Consider enabling only essential components.".to_string());
        }

        if enabled.len() > 15 {
            recommendations.push("Many components enabled. Consider lazy loading non-critical components.".to_string());
        }

        if enabled.contains(&"combobox") && enabled.contains(&"select") {
            recommendations.push("Both combobox and select enabled. Consider using only one for similar functionality.".to_string());
        }

        if enabled.contains(&"dialog") && enabled.contains(&"sheet") {
            recommendations.push("Both dialog and sheet enabled. Consider using only one modal component.".to_string());
        }

        recommendations
    }
}

/// Macro to conditionally include components
#[macro_export]
macro_rules! include_component {
    ($feature:expr, $component:expr) => {
        #[cfg(feature = $feature)]
        pub use $component;
        
        #[cfg(not(feature = $feature))]
        pub const $component: () = ();
    };
}

/// Macro to conditionally include component modules
#[macro_export]
macro_rules! include_component_module {
    ($feature:expr, $module:expr) => {
        #[cfg(feature = $feature)]
        pub mod $module;
        
        #[cfg(not(feature = $feature))]
        pub mod $module {
            // Empty module when feature is disabled
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_features() {
        let features = ComponentRegistry::available_features();
        assert!(features.contains(&"button"));
        assert!(features.contains(&"input"));
        assert!(features.len() >= 20);
    }

    #[test]
    fn test_feature_size_estimates() {
        assert!(ComponentRegistry::feature_size_estimate("button") > 0);
        assert!(ComponentRegistry::feature_size_estimate("dialog") > ComponentRegistry::feature_size_estimate("badge"));
    }

    #[test]
    fn test_optimization_recommendations() {
        let recommendations = ComponentRegistry::optimization_recommendations();
        // Should provide some recommendations
        assert!(!recommendations.is_empty());
    }
}
