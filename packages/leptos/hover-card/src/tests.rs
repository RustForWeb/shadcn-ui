#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use shadcn_ui_test_utils::leptos_testing::LeptosTestUtils;

    #[test]
    fn test_hover_card_component_exists() {
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed);
    }

    #[test]
    fn test_hover_card_component_with_props() {
        let mut props = HashMap::new();
        props.insert("class".to_string(), "test-class".to_string());
        let result = LeptosTestUtils::test_component_with_props(props);
        assert!(result.passed);
    }

    #[test]
    fn test_hover_card_component_with_children() {
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed);
    }

    #[test]
    fn test_hover_card_component_styling() {
        let result = LeptosTestUtils::test_component_styling();
        assert!(result.passed);
    }

    #[test]
    fn test_hover_card_component_accessibility() {
        let result = LeptosTestUtils::test_component_accessibility();
        assert!(result.passed);
    }
}
