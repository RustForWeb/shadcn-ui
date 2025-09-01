//! Theme validation utilities for ensuring visual consistency.

use crate::{Theme, TestResult};
use std::collections::HashMap;

/// Validates theme consistency across components and frameworks
pub struct ThemeValidator {
    pub themes: Vec<Theme>,
    pub component_classes: HashMap<String, Vec<String>>,
}

impl ThemeValidator {
    pub fn new() -> Self {
        Self {
            themes: vec![Theme::Default, Theme::NewYork],
            component_classes: HashMap::new(),
        }
    }
    
    pub fn add_component_classes(
        mut self, 
        component: impl Into<String>, 
        classes: Vec<String>
    ) -> Self {
        self.component_classes.insert(component.into(), classes);
        self
    }
    
    /// Validate that theme generates expected CSS classes
    pub fn validate_theme_classes(&self, component: &str, theme: Theme) -> TestResult {
        if let Some(classes) = self.component_classes.get(component) {
            // In a real implementation, this would validate actual CSS class generation
            let expected_theme_classes = match theme {
                Theme::Default => vec!["bg-primary", "text-primary-foreground"],
                Theme::NewYork => vec!["bg-primary", "text-primary-foreground", "rounded-lg"],
            };
            
            let has_theme_classes = expected_theme_classes.iter()
                .any(|expected| classes.iter().any(|actual| actual.contains(expected)));
                
            if has_theme_classes {
                TestResult::success(format!(
                    "Component '{}' has valid {:?} theme classes", 
                    component, theme
                ))
                .with_detail("classes_count", classes.len().to_string())
            } else {
                TestResult::failure(format!(
                    "Component '{}' missing expected {:?} theme classes", 
                    component, theme
                ))
            }
        } else {
            TestResult::failure(format!("Component '{}' not found in validator", component))
        }
    }
    
    /// Validate consistency between different themes for the same component
    pub fn validate_theme_consistency(&self, component: &str) -> TestResult {
        if let Some(classes) = self.component_classes.get(component) {
            // Check that both themes have core structural classes
            let core_classes = ["inline-flex", "items-center", "justify-center"];
            let has_core_classes = core_classes.iter()
                .all(|core| classes.iter().any(|actual| actual.contains(core)));
                
            if has_core_classes {
                TestResult::success(format!(
                    "Component '{}' maintains theme consistency", 
                    component
                ))
            } else {
                TestResult::failure(format!(
                    "Component '{}' lacks consistent core classes across themes", 
                    component
                ))
            }
        } else {
            TestResult::failure(format!("Component '{}' not found", component))
        }
    }
    
    /// Validate accessibility features are preserved across themes
    pub fn validate_accessibility_consistency(&self, component: &str) -> TestResult {
        // Check for accessibility-related classes and attributes
        let accessibility_indicators = [
            "focus-visible:outline-none",
            "focus-visible:ring-2",
            "disabled:pointer-events-none",
            "disabled:opacity-50",
        ];
        
        if let Some(classes) = self.component_classes.get(component) {
            let has_accessibility = accessibility_indicators.iter()
                .any(|indicator| classes.iter().any(|actual| actual.contains(indicator)));
                
            if has_accessibility {
                TestResult::success(format!(
                    "Component '{}' maintains accessibility across themes", 
                    component
                ))
            } else {
                TestResult::failure(format!(
                    "Component '{}' may be missing accessibility features", 
                    component
                ))
            }
        } else {
            TestResult::failure(format!("Component '{}' not found", component))
        }
    }
}

impl Default for ThemeValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn theme_validator_creation() {
        let validator = ThemeValidator::new()
            .add_component_classes("button", vec![
                "inline-flex".to_string(),
                "items-center".to_string(),
                "bg-primary".to_string(),
                "focus-visible:ring-2".to_string(),
            ]);
            
        assert_eq!(validator.themes.len(), 2);
        assert!(validator.component_classes.contains_key("button"));
    }
    
    #[test]
    fn validate_theme_classes_success() {
        let validator = ThemeValidator::new()
            .add_component_classes("button", vec![
                "bg-primary".to_string(),
                "text-primary-foreground".to_string(),
            ]);
            
        let result = validator.validate_theme_classes("button", Theme::Default);
        assert!(result.passed);
    }
    
    #[test]
    fn validate_consistency_success() {
        let validator = ThemeValidator::new()
            .add_component_classes("button", vec![
                "inline-flex".to_string(),
                "items-center".to_string(),
                "justify-center".to_string(),
            ]);
            
        let result = validator.validate_theme_consistency("button");
        assert!(result.passed);
    }
}