//! Cross-framework parity checking utilities.

use crate::{Framework, ParityResult};
// Remove unused imports
// use serde_json::{Value, Map};
use std::collections::HashMap;

/// Component API specification for cross-framework comparison
#[derive(Debug, Clone)]
pub struct ComponentSpec {
    pub name: String,
    pub props: HashMap<String, PropSpec>,
    pub events: Vec<String>,
    pub variants: Vec<String>,
    pub sizes: Vec<String>,
}

/// Property specification with type and requirements
#[derive(Debug, Clone)]
pub struct PropSpec {
    pub prop_type: String,
    pub required: bool,
    pub default_value: Option<String>,
}

/// Framework-specific component implementation details
#[derive(Debug, Clone)]
pub struct FrameworkImplementation {
    pub framework: Framework,
    pub component_spec: ComponentSpec,
    pub css_classes: Vec<String>,
    pub dependencies: Vec<String>,
}

/// Checks parity between framework implementations
pub struct ParityChecker {
    implementations: HashMap<Framework, FrameworkImplementation>,
}

impl ParityChecker {
    pub fn new() -> Self {
        Self {
            implementations: HashMap::new(),
        }
    }
    
    pub fn add_implementation(mut self, implementation: FrameworkImplementation) -> Self {
        self.implementations.insert(implementation.framework.clone(), implementation);
        self
    }
    
    /// Check API parity across all registered framework implementations
    pub fn check_api_parity(&self) -> ParityResult {
        if self.implementations.len() < 2 {
            return ParityResult::with_differences(vec![
                "Need at least 2 implementations to check parity".to_string()
            ]);
        }
        
        let mut differences = Vec::new();
        let frameworks: Vec<_> = self.implementations.keys().cloned().collect();
        
        // Compare each pair of implementations
        for i in 0..frameworks.len() {
            for j in (i + 1)..frameworks.len() {
                let impl1 = &self.implementations[&frameworks[i]];
                let impl2 = &self.implementations[&frameworks[j]];
                
                differences.extend(self.compare_props(&impl1.component_spec, &impl2.component_spec));
                differences.extend(self.compare_events(&impl1.component_spec, &impl2.component_spec));
                differences.extend(self.compare_variants(&impl1.component_spec, &impl2.component_spec));
            }
        }
        
        ParityResult::with_differences(differences)
    }
    
    /// Check theme consistency across implementations
    pub fn check_theme_parity(&self) -> ParityResult {
        let mut differences = Vec::new();
        let frameworks: Vec<_> = self.implementations.keys().cloned().collect();
        
        for i in 0..frameworks.len() {
            for j in (i + 1)..frameworks.len() {
                let impl1 = &self.implementations[&frameworks[i]];
                let impl2 = &self.implementations[&frameworks[j]];
                
                // Compare CSS class patterns
                let common_classes = self.find_common_classes(&impl1.css_classes, &impl2.css_classes);
                if common_classes.len() < 3 { // Expect at least 3 common structural classes
                    differences.push(format!(
                        "Insufficient common CSS classes between {:?} and {:?}: {}",
                        frameworks[i], frameworks[j], common_classes.len()
                    ));
                }
            }
        }
        
        ParityResult::with_differences(differences)
    }
    
    /// Check dependency consistency
    pub fn check_dependency_parity(&self) -> ParityResult {
        let mut differences = Vec::new();
        let frameworks: Vec<_> = self.implementations.keys().cloned().collect();
        
        // Collect all unique dependencies
        let mut all_deps: Vec<String> = self.implementations.values()
            .flat_map(|impl_| impl_.dependencies.iter())
            .cloned()
            .collect();
        all_deps.sort();
        all_deps.dedup();
        
        // Check if each framework has equivalent dependencies
        for framework in &frameworks {
            let impl_deps = &self.implementations[framework].dependencies;
            let missing_deps: Vec<_> = all_deps.iter()
                .filter(|dep| !impl_deps.contains(dep))
                .collect();
                
            if !missing_deps.is_empty() {
                differences.push(format!(
                    "{:?} missing dependencies: {:?}",
                    framework, missing_deps
                ));
            }
        }
        
        ParityResult::with_differences(differences)
    }
    
    fn compare_props(&self, spec1: &ComponentSpec, spec2: &ComponentSpec) -> Vec<String> {
        let mut differences = Vec::new();
        
        // Check for props in spec1 but not spec2
        for (prop_name, prop_spec) in &spec1.props {
            if let Some(other_prop) = spec2.props.get(prop_name) {
                if prop_spec.prop_type != other_prop.prop_type {
                    differences.push(format!(
                        "Property '{}' type mismatch: '{}' vs '{}'",
                        prop_name, prop_spec.prop_type, other_prop.prop_type
                    ));
                }
                if prop_spec.required != other_prop.required {
                    differences.push(format!(
                        "Property '{}' requirement mismatch: {} vs {}",
                        prop_name, prop_spec.required, other_prop.required
                    ));
                }
            } else {
                differences.push(format!("Property '{}' missing in second implementation", prop_name));
            }
        }
        
        // Check for props in spec2 but not spec1
        for prop_name in spec2.props.keys() {
            if !spec1.props.contains_key(prop_name) {
                differences.push(format!("Property '{}' missing in first implementation", prop_name));
            }
        }
        
        differences
    }
    
    fn compare_events(&self, spec1: &ComponentSpec, spec2: &ComponentSpec) -> Vec<String> {
        let mut differences = Vec::new();
        
        for event in &spec1.events {
            if !spec2.events.contains(event) {
                differences.push(format!("Event '{}' missing in second implementation", event));
            }
        }
        
        for event in &spec2.events {
            if !spec1.events.contains(event) {
                differences.push(format!("Event '{}' missing in first implementation", event));
            }
        }
        
        differences
    }
    
    fn compare_variants(&self, spec1: &ComponentSpec, spec2: &ComponentSpec) -> Vec<String> {
        let mut differences = Vec::new();
        
        if spec1.variants.len() != spec2.variants.len() {
            differences.push(format!(
                "Variant count mismatch: {} vs {}",
                spec1.variants.len(), spec2.variants.len()
            ));
        }
        
        for variant in &spec1.variants {
            if !spec2.variants.contains(variant) {
                differences.push(format!("Variant '{}' missing in second implementation", variant));
            }
        }
        
        differences
    }
    
    fn find_common_classes(&self, classes1: &[String], classes2: &[String]) -> Vec<String> {
        classes1.iter()
            .filter(|class1| classes2.iter().any(|class2| class1 == &class2))
            .cloned()
            .collect()
    }
}

impl Default for ParityChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_button_spec() -> ComponentSpec {
        let mut props = HashMap::new();
        props.insert("variant".to_string(), PropSpec {
            prop_type: "ButtonVariant".to_string(),
            required: false,
            default_value: Some("Default".to_string()),
        });
        props.insert("disabled".to_string(), PropSpec {
            prop_type: "bool".to_string(),
            required: false,
            default_value: Some("false".to_string()),
        });
        
        ComponentSpec {
            name: "Button".to_string(),
            props,
            events: vec!["on_click".to_string()],
            variants: vec!["Default".to_string(), "Primary".to_string(), "Secondary".to_string()],
            sizes: vec!["Small".to_string(), "Medium".to_string(), "Large".to_string()],
        }
    }
    
    #[test]
    fn parity_check_identical_specs() {
        let spec = create_button_spec();
        
        let yew_impl = FrameworkImplementation {
            framework: Framework::Yew,
            component_spec: spec.clone(),
            css_classes: vec!["btn".to_string(), "btn-primary".to_string()],
            dependencies: vec!["yew".to_string()],
        };
        
        let leptos_impl = FrameworkImplementation {
            framework: Framework::Leptos,
            component_spec: spec,
            css_classes: vec!["btn".to_string(), "btn-primary".to_string()],
            dependencies: vec!["leptos".to_string()],
        };
        
        let checker = ParityChecker::new()
            .add_implementation(yew_impl)
            .add_implementation(leptos_impl);
            
        let result = checker.check_api_parity();
        assert!(result.frameworks_match);
        assert_eq!(result.score, 1.0);
    }
}