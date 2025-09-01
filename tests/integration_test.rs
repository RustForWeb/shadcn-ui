//! Integration tests for shadcn-ui component implementations.

use shadcn_ui_test_utils::{
    ComponentTester, ComponentComparator, ThemeValidator, ParityChecker,
    Framework, Theme, FrameworkImplementation, ComponentSpec, PropSpec
};
use std::collections::HashMap;

#[cfg(test)]
mod component_parity_tests {
    use super::*;

    #[test]
    fn test_button_cross_framework_parity() {
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

        let button_spec = ComponentSpec {
            name: "Button".to_string(),
            props,
            events: vec!["on_click".to_string()],
            variants: vec!["Default".to_string(), "Primary".to_string(), "Secondary".to_string()],
            sizes: vec!["Small".to_string(), "Medium".to_string(), "Large".to_string()],
        };

        let yew_impl = FrameworkImplementation {
            framework: Framework::Yew,
            component_spec: button_spec.clone(),
            css_classes: vec![
                "inline-flex".to_string(),
                "items-center".to_string(), 
                "justify-center".to_string(),
                "bg-primary".to_string(),
                "text-primary-foreground".to_string(),
            ],
            dependencies: vec!["yew".to_string(), "tailwind_fuse".to_string()],
        };

        let leptos_impl = FrameworkImplementation {
            framework: Framework::Leptos,
            component_spec: button_spec,
            css_classes: vec![
                "inline-flex".to_string(),
                "items-center".to_string(),
                "justify-center".to_string(), 
                "bg-primary".to_string(),
                "text-primary-foreground".to_string(),
            ],
            dependencies: vec!["leptos".to_string(), "tailwind_fuse".to_string()],
        };

        let checker = ParityChecker::new()
            .add_implementation(yew_impl)
            .add_implementation(leptos_impl);

        let api_result = checker.check_api_parity();
        assert!(api_result.frameworks_match, "Button API should match across frameworks");
        assert_eq!(api_result.score, 1.0, "Button should have perfect parity score");

        let theme_result = checker.check_theme_parity();
        assert!(theme_result.frameworks_match, "Button themes should match across frameworks");
    }

    #[test]
    fn test_checkbox_implementation_parity() {
        let mut props = HashMap::new();
        props.insert("checked".to_string(), PropSpec {
            prop_type: "bool".to_string(),
            required: false,
            default_value: Some("false".to_string()),
        });
        props.insert("disabled".to_string(), PropSpec {
            prop_type: "bool".to_string(),
            required: false,
            default_value: Some("false".to_string()),
        });

        let checkbox_spec = ComponentSpec {
            name: "Checkbox".to_string(),
            props,
            events: vec!["on_checked_change".to_string()],
            variants: vec![], // Checkbox doesn't have variants
            sizes: vec![], // Checkbox doesn't have size variations
        };

        let yew_impl = FrameworkImplementation {
            framework: Framework::Yew,
            component_spec: checkbox_spec.clone(),
            css_classes: vec![
                "peer".to_string(),
                "h-4".to_string(),
                "w-4".to_string(),
                "rounded-sm".to_string(),
                "border".to_string(),
                "border-primary".to_string(),
            ],
            dependencies: vec!["yew".to_string(), "tailwind_fuse".to_string(), "web-sys".to_string()],
        };

        let leptos_impl = FrameworkImplementation {
            framework: Framework::Leptos,
            component_spec: checkbox_spec,
            css_classes: vec![
                "peer".to_string(),
                "h-4".to_string(),
                "w-4".to_string(),
                "rounded-sm".to_string(),
                "border".to_string(),
                "border-primary".to_string(),
            ],
            dependencies: vec!["leptos".to_string(), "tailwind_fuse".to_string(), "web-sys".to_string()],
        };

        let checker = ParityChecker::new()
            .add_implementation(yew_impl)
            .add_implementation(leptos_impl);

        let api_result = checker.check_api_parity();
        assert!(api_result.frameworks_match, "Checkbox API should match across frameworks");

        let theme_result = checker.check_theme_parity();
        assert!(theme_result.frameworks_match, "Checkbox themes should match across frameworks");
    }
}

#[cfg(test)]
mod theme_validation_tests {
    use super::*;

    #[test]
    fn test_theme_consistency_validation() {
        let validator = ThemeValidator::new()
            .add_component_classes("button", vec![
                "inline-flex".to_string(),
                "items-center".to_string(),
                "justify-center".to_string(),
                "bg-primary".to_string(),
                "text-primary-foreground".to_string(),
                "focus-visible:ring-2".to_string(),
                "disabled:opacity-50".to_string(),
            ]);

        let default_result = validator.validate_theme_classes("button", Theme::Default);
        assert!(default_result.passed, "Button should have valid default theme classes");

        let new_york_result = validator.validate_theme_classes("button", Theme::NewYork);
        assert!(new_york_result.passed, "Button should have valid New York theme classes");

        let consistency_result = validator.validate_theme_consistency("button");
        assert!(consistency_result.passed, "Button should maintain theme consistency");

        let accessibility_result = validator.validate_accessibility_consistency("button");
        assert!(accessibility_result.passed, "Button should maintain accessibility features");
    }
}

#[cfg(test)]
mod component_testing {
    use super::*;

    #[test]
    fn test_component_tester_framework_comparison() {
        let comparator = ComponentComparator::new("button")
            .add_framework(Framework::Yew, Theme::Default)
            .add_framework(Framework::Leptos, Theme::Default);

        let result = comparator.compare_frameworks();
        assert!(result.score > 0.8, "Framework implementations should have high similarity");
    }

    #[test]
    fn test_individual_component_validation() {
        let yew_tester = ComponentTester::new("button", Framework::Yew)
            .with_theme(Theme::Default)
            .with_property("variant", "primary")
            .with_property("disabled", "false");

        let rendering_result = yew_tester.test_rendering();
        assert!(rendering_result.passed, "Yew button should render successfully");

        let interaction_result = yew_tester.test_interactions();
        assert!(interaction_result.passed, "Yew button interactions should work");

        let accessibility_result = yew_tester.test_accessibility();
        assert!(accessibility_result.passed, "Yew button should be accessible");

        let theme_result = yew_tester.test_theme_consistency(Theme::NewYork);
        assert!(theme_result.passed, "Yew button themes should be consistent");
    }
}

#[test]
fn test_registry_completeness() {
    // This test validates that our registry contains all expected components
    use shadcn_registry::{FrameworkName, UI};
    
    let yew_registry = UI.get(&FrameworkName::Yew).expect("Yew registry should exist");
    let leptos_registry = UI.get(&FrameworkName::Leptos).expect("Leptos registry should exist");
    
    // Test that both registries have button and checkbox components
    let yew_has_button = yew_registry.iter().any(|entry| entry.name == "button");
    let yew_has_checkbox = yew_registry.iter().any(|entry| entry.name == "checkbox");
    
    let leptos_has_button = leptos_registry.iter().any(|entry| entry.name == "button");
    let leptos_has_checkbox = leptos_registry.iter().any(|entry| entry.name == "checkbox");
    
    assert!(yew_has_button, "Yew registry should contain button component");
    assert!(yew_has_checkbox, "Yew registry should contain checkbox component");
    assert!(leptos_has_button, "Leptos registry should contain button component");
    assert!(leptos_has_checkbox, "Leptos registry should contain checkbox component");
    
    println!("✅ Registry validation passed");
    println!("   Yew components: {}", yew_registry.len());
    println!("   Leptos components: {}", leptos_registry.len());
}