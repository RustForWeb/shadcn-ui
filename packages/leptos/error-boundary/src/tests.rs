#[cfg(test)]
mod tests {
    use super::*;
    use shadcn_ui_test_utils::LeptosTestUtils;

    #[test]
    fn test_error_fallback_renders() {
        let result = LeptosTestUtils::test_component_renders::<ErrorFallback>();
        assert!(result.passed);
    }

    #[test]
    fn test_error_fallback_with_props() {
        let error_info = ErrorInfo {
            message: "Test error message".to_string(),
            technical_details: Some("Technical details".to_string()),
        };
        
        let result = LeptosTestUtils::test_component_with_props::<ErrorFallback>(error_info);
        assert!(result.passed);
    }

    #[test]
    fn test_error_fallback_accessibility() {
        let result = LeptosTestUtils::test_component_accessibility::<ErrorFallback>();
        assert!(result.passed);
    }

    #[test]
    fn test_error_fallback_styling() {
        let result = LeptosTestUtils::test_component_styling::<ErrorFallback>();
        assert!(result.passed);
    }

    #[test]
    fn test_error_boundary_renders() {
        let result = LeptosTestUtils::test_component_renders::<ErrorBoundary>();
        assert!(result.passed);
    }

    #[test]
    fn test_error_info_creation() {
        let error_info = ErrorInfo {
            message: "Test error".to_string(),
            technical_details: Some("Technical details".to_string()),
        };
        
        assert_eq!(error_info.message, "Test error");
        assert_eq!(error_info.technical_details, Some("Technical details".to_string()));
    }

    #[test]
    fn test_create_user_error() {
        let error = create_user_error("User message", Some("Technical"));
        assert_eq!(error.message, "User message");
        assert_eq!(error.technical_details, Some("Technical".to_string()));
        
        let error_no_tech = create_user_error("User message", None);
        assert_eq!(error_no_tech.message, "User message");
        assert_eq!(error_no_tech.technical_details, None);
    }

    #[test]
    fn test_handle_error() {
        let result: Result<i32, &str> = Ok(42);
        assert_eq!(handle_error(result), Some(42));
        
        let error_result: Result<i32, &str> = Err("Error");
        assert_eq!(handle_error(error_result), None);
    }

    #[test]
    fn test_error_constants() {
        // Test that our styling constants are defined
        assert!(!ERROR_FALLBACK_CLASSES.is_empty());
        assert!(!ERROR_CONTENT_CLASSES.is_empty());
        assert!(!ERROR_ICON_CLASSES.is_empty());
        assert!(!ERROR_TITLE_CLASSES.is_empty());
        assert!(!ERROR_MESSAGE_CLASSES.is_empty());
        assert!(!ERROR_ACTIONS_CLASSES.is_empty());
        assert!(!ERROR_RETRY_CLASSES.is_empty());
        
        // Test that our message constants are defined
        assert!(!DEFAULT_ERROR_TITLE.is_empty());
        assert!(!DEFAULT_ERROR_MESSAGE.is_empty());
        assert!(!DEFAULT_RETRY_TEXT.is_empty());
        
        // Test that our icon constants are defined
        assert!(!DEFAULT_ERROR_ICON.is_empty());
        assert!(!DEFAULT_ERROR_ICON_ALT.is_empty());
        
        // Test that our configuration constants have sensible values
        assert!(DEFAULT_LOG_ERRORS);
        assert!(!DEFAULT_SHOW_TECHNICAL_DETAILS);
        
        // Test that our styling constants contain CSS
        assert!(ERROR_FALLBACK_STYLES.contains(".error-fallback"));
        assert!(ERROR_FALLBACK_STYLES.contains("background-color"));
        assert!(ERROR_FALLBACK_STYLES.contains("border-radius"));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_error_handler_hook() {
        // Test the error handler hook in a browser environment
        let (has_error, set_has_error, set_error_info) = use_error_handler();
        
        // Initially no error
        assert!(!has_error.get());
        
        // Set an error
        let test_error = ErrorInfo {
            message: "Hook test error".to_string(),
            technical_details: None,
        };
        
        set_error_info.set(Some(test_error));
        set_has_error.set(true);
        
        // Check that error was set
        assert!(has_error.get());
        
        let current_error = set_error_info.get();
        assert!(current_error.is_some());
        
        if let Some(error) = current_error {
            assert_eq!(error.message, "Hook test error");
        }
        
        // Clear error
        set_has_error.set(false);
        set_error_info.set(None);
        assert!(!has_error.get());
    }
}
