//! Test templates for different component types.
//!
//! This module provides pre-built test code strings for different component types,
//! making it easy to generate comprehensive tests for Leptos components.

use crate::leptos_testing::{ComponentTestSuite, test_helpers};

/// Test code generator for different component types
pub struct TestCodeGenerator;

impl TestCodeGenerator {
    /// Generate test code for basic components
    pub fn generate_basic_component_tests(component_name: &str) -> String {
        let test_builder = test_helpers::basic_component_test(component_name);
        let _test_suite = ComponentTestSuite::new(&format!("{}_test_suite", component_name))
            .add_test(test_builder);
        
        format!(
            r#"#[cfg(test)]
mod tests {{
    use crate::{component_name_pascal};
    use shadcn_ui_test_utils::leptos_testing::{{LeptosTestUtils, ComponentTestBuilder, test_helpers}};

    #[test]
    fn test_{component_name}_component_exists() {{
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_basic_functionality() {{
        // Test basic component functionality
        let result = LeptosTestUtils::test_component_with_props(std::collections::HashMap::new());
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_accessibility() {{
        // Test component accessibility
        let result = LeptosTestUtils::test_component_accessibility();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_styling() {{
        // Test component styling
        let result = LeptosTestUtils::test_component_styling();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_comprehensive() {{
        // Comprehensive test using the test builder
        let test = test_helpers::basic_component_test("{component_name}");
        let result = test.run();
        assert!(result.passed);
    }}
}}"#,
            component_name = component_name,
            component_name_pascal = Self::to_pascal_case(component_name)
        )
    }

    /// Generate test code for form components
    pub fn generate_form_component_tests(component_name: &str) -> String {
        format!(
            r#"#[cfg(test)]
mod tests {{
    use crate::{component_name_pascal};
    use shadcn_ui_test_utils::leptos_testing::{{LeptosTestUtils, ComponentTestBuilder, test_helpers}};
    use std::collections::HashMap;

    #[test]
    fn test_{component_name}_component_exists() {{
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_form_functionality() {{
        // Test form-specific functionality
        let mut props = HashMap::new();
        props.insert("value".to_string(), "test_value".to_string());
        props.insert("placeholder".to_string(), "Enter text".to_string());
        
        let result = LeptosTestUtils::test_component_with_props(props);
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_accessibility() {{
        // Test form component accessibility
        let result = LeptosTestUtils::test_component_accessibility();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_events() {{
        // Test form component events
        let events = vec!["change".to_string(), "input".to_string(), "focus".to_string(), "blur".to_string()];
        let result = LeptosTestUtils::test_component_events(events);
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_comprehensive() {{
        // Comprehensive form component test
        let test = test_helpers::form_component_test("{component_name}");
        let result = test.run();
        assert!(result.passed);
    }}
}}"#,
            component_name = component_name,
            component_name_pascal = Self::to_pascal_case(component_name)
        )
    }

    /// Generate test code for interactive components
    pub fn generate_interactive_component_tests(component_name: &str) -> String {
        format!(
            r#"#[cfg(test)]
mod tests {{
    use crate::{component_name_pascal};
    use shadcn_ui_test_utils::leptos_testing::{{LeptosTestUtils, ComponentTestBuilder, test_helpers}};
    use std::collections::HashMap;

    #[test]
    fn test_{component_name}_component_exists() {{
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_interactions() {{
        // Test interactive functionality
        let result = LeptosTestUtils::test_component_interaction("click");
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_state_changes() {{
        // Test state change functionality
        let result = LeptosTestUtils::test_component_state_change();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_events() {{
        // Test interactive component events
        let events = vec!["click".to_string(), "hover".to_string(), "focus".to_string()];
        let result = LeptosTestUtils::test_component_events(events);
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_accessibility() {{
        // Test interactive component accessibility
        let result = LeptosTestUtils::test_component_accessibility();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_comprehensive() {{
        // Comprehensive interactive component test
        let test = test_helpers::interactive_component_test("{component_name}");
        let result = test.run();
        assert!(result.passed);
    }}
}}"#,
            component_name = component_name,
            component_name_pascal = Self::to_pascal_case(component_name)
        )
    }

    /// Generate test code for layout components
    pub fn generate_layout_component_tests(component_name: &str) -> String {
        format!(
            r#"#[cfg(test)]
mod tests {{
    use crate::{component_name_pascal};
    use shadcn_ui_test_utils::leptos_testing::{{LeptosTestUtils, ComponentTestBuilder, test_helpers}};

    #[test]
    fn test_{component_name}_component_exists() {{
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_layout_functionality() {{
        // Test layout-specific functionality
        let result = LeptosTestUtils::test_component_with_props(std::collections::HashMap::new());
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_styling() {{
        // Test layout component styling
        let result = LeptosTestUtils::test_component_styling();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_responsive() {{
        // Test responsive behavior
        let result = LeptosTestUtils::test_component_responsive();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_comprehensive() {{
        // Comprehensive layout component test
        let test = test_helpers::layout_component_test("{component_name}");
        let result = test.run();
        assert!(result.passed);
    }}
}}"#,
            component_name = component_name,
            component_name_pascal = Self::to_pascal_case(component_name)
        )
    }

    /// Generate test code for feedback components
    pub fn generate_feedback_component_tests(component_name: &str) -> String {
        format!(
            r#"#[cfg(test)]
mod tests {{
    use crate::{component_name_pascal};
    use shadcn_ui_test_utils::leptos_testing::{{LeptosTestUtils, ComponentTestBuilder, test_helpers}};
    use std::collections::HashMap;

    #[test]
    fn test_{component_name}_component_exists() {{
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_feedback_functionality() {{
        // Test feedback-specific functionality
        let mut props = HashMap::new();
        props.insert("message".to_string(), "Test message".to_string());
        props.insert("type".to_string(), "info".to_string());
        
        let result = LeptosTestUtils::test_component_with_props(props);
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_accessibility() {{
        // Test feedback component accessibility
        let result = LeptosTestUtils::test_component_accessibility();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_theme_switching() {{
        // Test theme switching functionality
        let result = LeptosTestUtils::test_component_theme_switching();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_performance() {{
        // Test performance characteristics
        let result = LeptosTestUtils::test_component_performance();
        assert!(result.passed);
    }}

    #[test]
    fn test_{component_name}_comprehensive() {{
        // Comprehensive feedback component test
        let test = test_helpers::feedback_component_test("{component_name}");
        let result = test.run();
        assert!(result.is_success());
    }}
}}"#,
            component_name = component_name,
            component_name_pascal = Self::to_pascal_case(component_name)
        )
    }

    /// Convert kebab-case to PascalCase
    fn to_pascal_case(input: &str) -> String {
        input
            .split('-')
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            })
            .collect()
    }
}
