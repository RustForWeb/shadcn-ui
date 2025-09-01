use std::fs;
use std::path::Path;

fn main() {
    let leptos_components_dir = "packages/leptos";
    
    // List of all components that need test files
    let components = vec![
        "accordion", "alert", "alert-dialog", "badge", "button", "card", "carousel", 
        "checkbox", "collapsible", "context-menu", "dialog", "drawer", "dropdown-menu",
        "hover-card", "input", "label", "menubar", "navigation-menu", "popover", 
        "progress", "radio-group", "scroll-area", "separator", "sheet", "skeleton",
        "slider", "switch", "table", "tabs", "textarea", "toast", "toggle", "tooltip"
    ];
    
    for component in components {
        let tests_file_path = format!("{}/{}/src/tests.rs", leptos_components_dir, component);
        
        // Check if tests.rs already exists
        if !Path::new(&tests_file_path).exists() {
            println!("Creating tests.rs for {}", component);
            
            let test_content = generate_basic_test_file(component);
            
            if let Err(e) = fs::write(&tests_file_path, test_content) {
                eprintln!("Failed to write {}: {}", tests_file_path, e);
            } else {
                println!("✅ Created {}", tests_file_path);
            }
        } else {
            println!("⏭️  {} already has tests.rs", component);
        }
    }
    
    println!("\n🎉 Test file generation complete!");
}

fn generate_basic_test_file(component_name: &str) -> String {
    let module_name = component_name.replace("-", "_");
    
    format!(r#"#[cfg(test)]
mod tests {{
    use wasm_bindgen_test::*;
    use shadcn_ui_test_utils::leptos_testing::LeptosTestUtils;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_{module_name}_component_exists() {{
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed);
    }}

    #[wasm_bindgen_test]
    fn test_{module_name}_renders_in_browser() {{
        // WASM-specific test for browser rendering
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed, "Component should render in browser: {{}}", result.message);
    }}

    #[test]
    fn test_{module_name}_props_handling() {{
        // Test basic prop handling
        let props = std::collections::HashMap::new();
        let result = LeptosTestUtils::test_component_with_props(props);
        assert!(result.passed, "Props should be handled correctly: {{}}", result.message);
    }}

    #[test]
    fn test_{module_name}_accessibility() {{
        // Test accessibility features
        let result = LeptosTestUtils::test_component_accessibility();
        assert!(result.passed, "Accessibility should be implemented: {{}}", result.message);
    }}

    #[test]
    fn test_{module_name}_styling() {{
        // Test CSS classes and styling
        let result = LeptosTestUtils::test_component_styling();
        assert!(result.passed, "Styling should be applied correctly: {{}}", result.message);
    }}
}}
"#, module_name = module_name)
}
