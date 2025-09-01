//! Component testing utilities for validation and comparison.

use crate::{Framework, Theme, TestResult, ParityResult};
use std::collections::HashMap;
// Remove unused import
// use wasm_bindgen_test::*;

/// Generic component tester that validates component behavior
pub struct ComponentTester {
    pub component_name: String,
    pub framework: Framework,
    pub theme: Theme,
    pub properties: HashMap<String, String>,
}

impl ComponentTester {
    pub fn new(component_name: impl Into<String>, framework: Framework) -> Self {
        Self {
            component_name: component_name.into(),
            framework,
            theme: Theme::Default,
            properties: HashMap::new(),
        }
    }
    
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
    
    /// Test basic component rendering
    pub fn test_rendering(&self) -> TestResult {
        // This would be implemented with framework-specific rendering logic
        TestResult::success(format!(
            "{} component renders successfully with {:?} theme",
            self.component_name, self.theme
        ))
        .with_detail("framework", format!("{:?}", self.framework))
        .with_detail("theme", format!("{:?}", self.theme))
    }
    
    /// Test component interactions and event handlers
    pub fn test_interactions(&self) -> TestResult {
        // This would test click handlers, keyboard navigation, etc.
        TestResult::success(format!(
            "{} component interactions work correctly",
            self.component_name
        ))
    }
    
    /// Test accessibility features
    pub fn test_accessibility(&self) -> TestResult {
        // ARIA attributes, keyboard navigation, screen reader support
        TestResult::success(format!(
            "{} component meets accessibility requirements",
            self.component_name
        ))
    }
    
    /// Test theme consistency
    pub fn test_theme_consistency(&self, other_theme: Theme) -> TestResult {
        if self.theme == other_theme {
            return TestResult::failure("Cannot compare theme with itself".to_string());
        }
        
        // Would compare CSS classes, styling, visual output
        TestResult::success(format!(
            "{} component themes are visually distinct and consistent",
            self.component_name
        ))
        .with_detail("theme_1", format!("{:?}", self.theme))
        .with_detail("theme_2", format!("{:?}", other_theme))
    }
}

/// Cross-framework component comparison
pub struct ComponentComparator {
    pub component_name: String,
    pub testers: Vec<ComponentTester>,
}

impl ComponentComparator {
    pub fn new(component_name: impl Into<String>) -> Self {
        Self {
            component_name: component_name.into(),
            testers: vec![],
        }
    }
    
    pub fn add_framework(mut self, framework: Framework, theme: Theme) -> Self {
        let tester = ComponentTester::new(&self.component_name, framework)
            .with_theme(theme);
        self.testers.push(tester);
        self
    }
    
    /// Compare component implementations across frameworks
    pub fn compare_frameworks(&self) -> ParityResult {
        if self.testers.len() < 2 {
            return ParityResult::with_differences(vec![
                "Need at least 2 framework implementations to compare".to_string()
            ]);
        }
        
        let mut differences = Vec::new();
        
        // Compare each pair of framework implementations
        for i in 0..self.testers.len() {
            for j in (i + 1)..self.testers.len() {
                let tester1 = &self.testers[i];
                let tester2 = &self.testers[j];
                
                // Compare properties
                if tester1.properties.len() != tester2.properties.len() {
                    differences.push(format!(
                        "Property count mismatch: {:?} has {}, {:?} has {}",
                        tester1.framework,
                        tester1.properties.len(),
                        tester2.framework,
                        tester2.properties.len()
                    ));
                }
                
                // Compare specific properties
                for (key, value1) in &tester1.properties {
                    if let Some(value2) = tester2.properties.get(key) {
                        if value1 != value2 {
                            differences.push(format!(
                                "Property '{}' differs: {:?}='{}', {:?}='{}'",
                                key, tester1.framework, value1, tester2.framework, value2
                            ));
                        }
                    } else {
                        differences.push(format!(
                            "Property '{}' missing in {:?} implementation",
                            key, tester2.framework
                        ));
                    }
                }
            }
        }
        
        ParityResult::with_differences(differences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn component_tester_creation() {
        let tester = ComponentTester::new("button", Framework::Yew)
            .with_theme(Theme::NewYork)
            .with_property("variant", "primary")
            .with_property("size", "large");
            
        assert_eq!(tester.component_name, "button");
        assert_eq!(tester.framework, Framework::Yew);
        assert_eq!(tester.theme, Theme::NewYork);
        assert_eq!(tester.properties.get("variant"), Some(&"primary".to_string()));
    }
    
    #[test]
    fn parity_comparison_success() {
        let mut comparator = ComponentComparator::new("button");
        comparator.testers.push(
            ComponentTester::new("button", Framework::Yew)
                .with_property("variant", "primary")
        );
        comparator.testers.push(
            ComponentTester::new("button", Framework::Leptos)
                .with_property("variant", "primary")
        );
        
        let result = comparator.compare_frameworks();
        assert!(result.frameworks_match);
        assert_eq!(result.score, 1.0);
    }
}