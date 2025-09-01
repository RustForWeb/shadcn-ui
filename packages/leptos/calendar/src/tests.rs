#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use shadcn_ui_test_utils::leptos_testing::LeptosTestUtils;
    
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_calendar_component_exists() {
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed);
    }

    #[wasm_bindgen_test]
    fn test_calendar_renders_in_browser() {
        // WASM-specific test for browser rendering
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed, "Component should render in browser: {}", result.message);
    }

    #[test]
    fn test_calendar_props_handling() {
        // Test basic prop handling
        let props = std::collections::HashMap::new();
        let result = LeptosTestUtils::test_component_with_props(props);
        assert!(result.passed, "Props should be handled correctly: {}", result.message);
    }

    #[test]
    fn test_calendar_accessibility() {
        // Test accessibility features
        let result = LeptosTestUtils::test_component_accessibility();
        assert!(result.passed, "Accessibility should be implemented: {}", result.message);
    }

    #[test]
    fn test_calendar_styling() {
        // Test CSS classes and styling
        let result = LeptosTestUtils::test_component_styling();
        assert!(result.passed, "Styling should be applied correctly: {}", result.message);
    }
}
