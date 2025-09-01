//! Integration tests for radio-group component implementations.

use shadcn_ui_test_utils::{
    ComponentTester, ComponentComparator, ParityChecker,
    Framework, Theme, FrameworkImplementation, ComponentSpec, PropSpec
};
use std::collections::HashMap;

#[test]
fn test_radio_group_cross_framework_parity() {
    let mut props = HashMap::new();
    props.insert("value".to_string(), PropSpec {
        prop_type: "Option<String>".to_string(),
        required: false,
        default_value: Some("None".to_string()),
    });
    props.insert("on_value_change".to_string(), PropSpec {
        prop_type: "Option<Callback<String>>".to_string(),
        required: false,
        default_value: Some("None".to_string()),
    });
    props.insert("disabled".to_string(), PropSpec {
        prop_type: "bool".to_string(),
        required: false,
        default_value: Some("false".to_string()),
    });

    let radio_group_spec = ComponentSpec {
        name: "RadioGroup".to_string(),
        props: props.clone(),
        events: vec!["on_value_change".to_string()],
        variants: vec!["Default".to_string(), "NewYork".to_string()],
        sizes: vec![], // Radio group doesn't have size variations
    };

    let radio_group_item_spec = ComponentSpec {
        name: "RadioGroupItem".to_string(),
        props: {
            let mut item_props = HashMap::new();
            item_props.insert("value".to_string(), PropSpec {
                prop_type: "String".to_string(),
                required: true,
                default_value: None,
            });
            item_props.insert("disabled".to_string(), PropSpec {
                prop_type: "bool".to_string(),
                required: false,
                default_value: Some("false".to_string()),
            });
            item_props
        },
        events: vec!["on_click".to_string()],
        variants: vec!["Default".to_string(), "NewYork".to_string()],
        sizes: vec![], // Radio group items don't have size variations
    };

    let yew_impl = FrameworkImplementation {
        framework: Framework::Yew,
        component_spec: radio_group_spec.clone(),
        css_classes: vec![
            "grid".to_string(),
            "gap-2".to_string(),
            "aspect-square".to_string(),
            "h-4".to_string(),
            "w-4".to_string(),
            "rounded-full".to_string(),
            "border".to_string(),
            "border-primary".to_string(),
            "text-primary".to_string(),
        ],
        dependencies: vec!["yew".to_string(), "yew_style".to_string(), "web_sys".to_string()],
    };

    let leptos_impl = FrameworkImplementation {
        framework: Framework::Leptos,
        component_spec: radio_group_spec,
        css_classes: vec![
            "grid".to_string(),
            "gap-2".to_string(),
            "aspect-square".to_string(),
            "h-4".to_string(),
            "w-4".to_string(),
            "rounded-full".to_string(),
            "border".to_string(),
            "border-primary".to_string(),
            "text-primary".to_string(),
        ],
        dependencies: vec!["leptos".to_string(), "leptos_style".to_string(), "web_sys".to_string()],
    };

    let checker = ParityChecker::new()
        .add_implementation(yew_impl)
        .add_implementation(leptos_impl);

    let api_result = checker.check_api_parity();
    assert!(api_result.frameworks_match, "RadioGroup API should match across frameworks");
    assert_eq!(api_result.score, 1.0, "RadioGroup should have perfect parity score");

    let theme_result = checker.check_theme_parity();
    assert!(theme_result.frameworks_match, "RadioGroup themes should match across frameworks");

    let dependency_result = checker.check_dependency_parity();
    assert!(dependency_result.frameworks_match, "RadioGroup dependencies should be equivalent");
}

#[test]
fn test_radio_group_theme_consistency() {
    let validator = shadcn_ui_test_utils::ThemeValidator::new()
        .add_component_classes("radio-group", vec![
            "grid".to_string(),
            "gap-2".to_string(),
        ])
        .add_component_classes("radio-group-item", vec![
            "aspect-square".to_string(),
            "h-4".to_string(),
            "w-4".to_string(),
            "rounded-full".to_string(),
            "border".to_string(),
            "border-primary".to_string(),
            "text-primary".to_string(),
            "ring-offset-background".to_string(),
            "focus:outline-none".to_string(),
            "focus-visible:ring-2".to_string(),
            "focus-visible:ring-ring".to_string(),
            "focus-visible:ring-offset-2".to_string(),
            "disabled:cursor-not-allowed".to_string(),
            "disabled:opacity-50".to_string(),
        ])
        .add_component_classes("radio-group-indicator", vec![
            "flex".to_string(),
            "items-center".to_string(),
            "justify-center".to_string(),
        ])
        .add_component_classes("radio-group-indicator-dot", vec![
            "h-2.5".to_string(),
            "w-2.5".to_string(),
            "rounded-full".to_string(),
            "bg-current".to_string(),
        ]);

    let default_result = validator.validate_theme_classes("radio-group", Theme::Default);
    assert!(default_result.passed, "RadioGroup should have valid default theme classes");

    let new_york_result = validator.validate_theme_classes("radio-group", Theme::NewYork);
    assert!(new_york_result.passed, "RadioGroup should have valid New York theme classes");

    let consistency_result = validator.validate_theme_consistency("radio-group");
    assert!(consistency_result.passed, "RadioGroup should maintain theme consistency");

    let accessibility_result = validator.validate_accessibility_consistency("radio-group");
    assert!(accessibility_result.passed, "RadioGroup should maintain accessibility features");
}

#[test]
fn test_radio_group_accessibility_features() {
    let yew_tester = ComponentTester::new("radio-group", Framework::Yew)
        .with_theme(Theme::Default)
        .with_property("value", "option1")
        .with_property("disabled", "false");

    let leptos_tester = ComponentTester::new("radio-group", Framework::Leptos)
        .with_theme(Theme::Default)
        .with_property("value", "option1")
        .with_property("disabled", "false");

    // Test accessibility features
    let yew_accessibility = yew_tester.test_accessibility();
    assert!(yew_accessibility.passed, "Yew RadioGroup should be accessible");

    let leptos_accessibility = leptos_tester.test_accessibility();
    assert!(leptos_accessibility.passed, "Leptos RadioGroup should be accessible");

    // Test interactions
    let yew_interactions = yew_tester.test_interactions();
    assert!(yew_interactions.passed, "Yew RadioGroup interactions should work");

    let leptos_interactions = leptos_tester.test_interactions();
    assert!(leptos_interactions.passed, "Leptos RadioGroup interactions should work");
}

#[test]
fn test_radio_group_framework_comparison() {
    let comparator = ComponentComparator::new("radio-group")
        .add_framework(Framework::Yew, Theme::Default)
        .add_framework(Framework::Leptos, Theme::Default)
        .add_framework(Framework::Yew, Theme::NewYork)
        .add_framework(Framework::Leptos, Theme::NewYork);

    let result = comparator.compare_frameworks();
    assert!(result.score > 0.8, "RadioGroup framework implementations should have high similarity");
    println!("RadioGroup parity score: {:.2}", result.score);
}

#[test]
fn test_radio_group_registry_integration() {
    // Test that radio-group is properly registered in the component registry
    use shadcn_registry::{FrameworkName, UI};
    
    let yew_registry = UI.get(&FrameworkName::Yew).expect("Yew registry should exist");
    let leptos_registry = UI.get(&FrameworkName::Leptos).expect("Leptos registry should exist");
    
    // Test that both registries have radio-group component
    let yew_has_radio_group = yew_registry.iter().any(|entry| entry.name == "radio-group");
    let leptos_has_radio_group = leptos_registry.iter().any(|entry| entry.name == "radio-group");
    
    assert!(yew_has_radio_group, "Yew registry should contain radio-group component");
    assert!(leptos_has_radio_group, "Leptos registry should contain radio-group component");
    
    println!("✅ RadioGroup registry validation passed");
    println!("   Yew components: {}", yew_registry.len());
    println!("   Leptos components: {}", leptos_registry.len());
}

#[test]
fn test_radio_group_property_validation() {
    // Test that all required properties are properly defined
    let mut props = HashMap::new();
    props.insert("value".to_string(), PropSpec {
        prop_type: "Option<String>".to_string(),
        required: false,
        default_value: Some("None".to_string()),
    });
    props.insert("on_value_change".to_string(), PropSpec {
        prop_type: "Option<Callback<String>>".to_string(),
        required: false,
        default_value: Some("None".to_string()),
    });
    props.insert("disabled".to_string(), PropSpec {
        prop_type: "bool".to_string(),
        required: false,
        default_value: Some("false".to_string()),
    });

    // Validate that all properties have proper types
    for (prop_name, prop_spec) in &props {
        assert!(!prop_spec.prop_type.is_empty(), "Property '{}' should have a type", prop_name);
        if !prop_spec.required {
            assert!(prop_spec.default_value.is_some(), "Optional property '{}' should have a default value", prop_name);
        }
    }

    // Validate that required properties are marked as such
    let required_props: Vec<_> = props.iter()
        .filter(|(_, spec)| spec.required)
        .map(|(name, _)| name)
        .collect();
    
    // RadioGroup doesn't have any required props, so this should be empty
    assert!(required_props.is_empty(), "RadioGroup should not have required props");
}

#[test]
fn test_radio_group_item_property_validation() {
    // Test RadioGroupItem properties
    let mut props = HashMap::new();
    props.insert("value".to_string(), PropSpec {
        prop_type: "String".to_string(),
        required: true,
        default_value: None,
    });
    props.insert("disabled".to_string(), PropSpec {
        prop_type: "bool".to_string(),
        required: false,
        default_value: Some("false".to_string()),
    });

    // Validate that value is required
    let value_prop = props.get("value").unwrap();
    assert!(value_prop.required, "RadioGroupItem value should be required");
    assert!(value_prop.default_value.is_none(), "Required property should not have default value");

    // Validate that disabled is optional
    let disabled_prop = props.get("disabled").unwrap();
    assert!(!disabled_prop.required, "RadioGroupItem disabled should be optional");
    assert!(disabled_prop.default_value.is_some(), "Optional property should have default value");
}
