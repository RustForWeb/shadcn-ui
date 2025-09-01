#[cfg(test)]
mod tests {
    use crate::{cn, cn_flexible};

    #[test]
    fn test_cn_basic_functionality() {
        // Test basic class combination
        let result = cn(&["bg-blue-500", "text-white", "p-4"]);
        assert_eq!(result, "bg-blue-500 text-white p-4");
        
        // Test single class
        let result = cn(&["bg-red-500"]);
        assert_eq!(result, "bg-red-500");
        
        // Test empty input
        let result = cn(&[]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_cn_tailwind_merging() {
        // Test Tailwind CSS class merging (p-4 should override p-2)
        let result = cn(&["p-2", "p-4"]);
        assert_eq!(result, "p-4");
        
        // Test margin merging
        let result = cn(&["m-2", "m-4"]);
        assert_eq!(result, "m-4");
        
        // Test padding merging
        let result = cn(&["px-2", "px-4", "px-6"]);
        assert_eq!(result, "px-6");
    }

    #[test]
    fn test_cn_complex_classes() {
        // Test complex class combinations
        let result = cn(&[
            "bg-blue-500",
            "hover:bg-blue-600",
            "focus:ring-2",
            "focus:ring-blue-500",
            "rounded-lg",
            "px-4",
            "py-2"
        ]);
        
        // The result should contain all classes, properly merged
        assert!(result.contains("bg-blue-500"));
        assert!(result.contains("hover:bg-blue-600"));
        assert!(result.contains("focus:ring-2"));
        assert!(result.contains("focus:ring-blue-500"));
        assert!(result.contains("rounded-lg"));
        assert!(result.contains("px-4"));
        assert!(result.contains("py-2"));
    }

    #[test]
    fn test_cn_flexible_basic_functionality() {
        // Test with string slices
        let result = cn_flexible(&["bg-green-500", "text-black"]);
        assert_eq!(result, "bg-green-500 text-black");
        
        // Test with String types
        let result = cn_flexible(&[
            String::from("bg-yellow-500"),
            String::from("text-white")
        ]);
        assert_eq!(result, "bg-yellow-500 text-white");
        
        // Test mixed types - convert all to String for consistency
        let result = cn_flexible(&[
            String::from("bg-purple-500"),
            String::from("text-white"),
            String::from("p-4")
        ]);
        assert_eq!(result, "bg-purple-500 text-white p-4");
    }

    #[test]
    fn test_cn_flexible_tailwind_merging() {
        // Test Tailwind CSS class merging with flexible types
        let result = cn_flexible(&[
            String::from("p-2"),
            String::from("p-4")
        ]);
        assert_eq!(result, "p-4");
        
        // Test with more complex merging
        let result = cn_flexible(&[
            String::from("m-2"),
            String::from("m-4"),
            String::from("m-6")
        ]);
        assert_eq!(result, "m-6");
    }

    #[test]
    fn test_cn_flexible_empty_and_single() {
        // Test empty input
        let result = cn_flexible::<&str>(&[]);
        assert_eq!(result, "");
        
        // Test single item
        let result = cn_flexible(&[String::from("bg-blue-500")]);
        assert_eq!(result, "bg-blue-500");
    }

    #[test]
    fn test_cn_flexible_with_whitespace() {
        // Test classes with existing whitespace
        let result = cn_flexible(&[
            String::from("bg-blue-500 "),
            String::from(" text-white"),
            String::from("  p-4  ")
        ]);
        
        // Should handle whitespace properly
        assert!(result.contains("bg-blue-500"));
        assert!(result.contains("text-white"));
        assert!(result.contains("p-4"));
    }

    #[test]
    fn test_function_equivalence() {
        // Test that cn and cn_flexible produce the same result for string slices
        let classes = &["bg-blue-500", "text-white", "p-4"];
        let cn_result = cn(classes);
        let cn_flexible_result = cn_flexible(classes);
        
        assert_eq!(cn_result, cn_flexible_result);
    }

    #[test]
    fn test_edge_cases() {
        // Test with very long class names
        let long_class = "bg-gradient-to-r-from-blue-500-to-purple-600-via-pink-500";
        let result = cn(&[long_class]);
        assert_eq!(result, long_class);
        
        // Test with special characters (if supported by Tailwind)
        let result = cn(&["hover:bg-[#ff0000]", "focus:ring-[#00ff00]"]);
        assert!(result.contains("hover:bg-[#ff0000]"));
        assert!(result.contains("focus:ring-[#00ff00]"));
    }
}
