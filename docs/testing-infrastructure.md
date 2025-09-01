# Testing Infrastructure Documentation

This document provides a comprehensive overview of the testing infrastructure for the Rust shadcn/ui project, with a focus on the radio-group component as an example.

## Overview

The testing infrastructure is designed to ensure component quality, cross-framework parity, and maintainability across all supported Rust web frameworks. It consists of multiple layers of testing:

1. **Unit Tests** - Framework-specific component tests
2. **Integration Tests** - Cross-framework parity and consistency tests
3. **Accessibility Tests** - ARIA compliance and usability tests
4. **Theme Tests** - Visual consistency across design variants
5. **Build Tests** - Compilation and dependency validation

## Architecture

### Test Utilities (`packages/test-utils`)

The test utilities package provides shared testing infrastructure:

```rust
use shadcn_ui_test_utils::{
    ComponentTester, ComponentComparator, ParityChecker,
    Framework, Theme, FrameworkImplementation, ComponentSpec, PropSpec
};
```

#### Core Components

- **`ComponentTester`** - Validates individual component behavior
- **`ComponentComparator`** - Compares implementations across frameworks
- **`ParityChecker`** - Ensures API and feature parity
- **`ThemeValidator`** - Validates theme consistency
- **`VisualRegression`** - Visual regression testing (planned)

### Framework Support

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Framework {
    Leptos,
    Yew,
    Dioxus, // Planned
}
```

### Theme Variants

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Default,
    NewYork,
}
```

## Testing Layers

### 1. Unit Tests

Unit tests are framework-specific and test individual component functionality.

#### Yew Unit Tests

```rust
#[wasm_bindgen_test]
fn test_radio_group_renders() {
    let rendered = yew::start_app_with_props::<RadioGroup>(RadioGroupProps {
        value: Some("option1".to_string()),
        on_value_change: None,
        disabled: false,
        // ... other props
    });

    let container = rendered.query_selector("[role='radiogroup']").unwrap();
    assert!(container.is_some(), "Radio group should have role='radiogroup'");
}
```

#### Leptos Unit Tests

```rust
#[wasm_bindgen_test]
fn test_radio_group_renders() {
    let (selected_value, _) = create_signal(None::<String>);
    
    let rendered = mount_to_body(|| {
        view! {
            <RadioGroup value=selected_value>
                <RadioGroupItem value="option1".to_string()>
                    "Option 1"
                </RadioGroupItem>
            </RadioGroup>
        }
    });

    let container = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("[role='radiogroup']")
        .unwrap();
    assert!(container.is_some(), "Radio group should have role='radiogroup'");
}
```

### 2. Integration Tests

Integration tests ensure cross-framework parity and consistency.

```rust
#[test]
fn test_radio_group_cross_framework_parity() {
    let mut props = HashMap::new();
    props.insert("value".to_string(), PropSpec {
        prop_type: "Option<String>".to_string(),
        required: false,
        default_value: Some("None".to_string()),
    });
    // ... more props

    let radio_group_spec = ComponentSpec {
        name: "RadioGroup".to_string(),
        props,
        events: vec!["on_value_change".to_string()],
        variants: vec!["Default".to_string(), "NewYork".to_string()],
        sizes: vec![],
    };

    let yew_impl = FrameworkImplementation {
        framework: Framework::Yew,
        component_spec: radio_group_spec.clone(),
        css_classes: vec![
            "grid".to_string(),
            "gap-2".to_string(),
            // ... more classes
        ],
        dependencies: vec!["yew".to_string(), "yew_style".to_string()],
    };

    let leptos_impl = FrameworkImplementation {
        framework: Framework::Leptos,
        component_spec: radio_group_spec,
        css_classes: vec![
            "grid".to_string(),
            "gap-2".to_string(),
            // ... more classes
        ],
        dependencies: vec!["leptos".to_string(), "leptos_style".to_string()],
    };

    let checker = ParityChecker::new()
        .add_implementation(yew_impl)
        .add_implementation(leptos_impl);

    let api_result = checker.check_api_parity();
    assert!(api_result.frameworks_match, "RadioGroup API should match across frameworks");
    assert_eq!(api_result.score, 1.0, "RadioGroup should have perfect parity score");
}
```

### 3. Accessibility Tests

Accessibility tests ensure ARIA compliance and usability.

```rust
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
}
```

### 4. Theme Tests

Theme tests validate visual consistency across design variants.

```rust
#[test]
fn test_radio_group_theme_consistency() {
    let validator = ThemeValidator::new()
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
            // ... more classes
        ]);

    let default_result = validator.validate_theme_classes("radio-group", Theme::Default);
    assert!(default_result.passed, "RadioGroup should have valid default theme classes");

    let new_york_result = validator.validate_theme_classes("radio-group", Theme::NewYork);
    assert!(new_york_result.passed, "RadioGroup should have valid New York theme classes");
}
```

## Test Categories

### Component-Specific Tests

Each component should include tests for:

1. **Rendering** - Component renders correctly
2. **Props** - All props work as expected
3. **Events** - Event handlers function properly
4. **State** - Component state management
5. **Styling** - CSS classes and themes
6. **Accessibility** - ARIA attributes and keyboard navigation
7. **Variants** - Different theme variants
8. **Edge Cases** - Error states, disabled states, etc.

### Cross-Framework Tests

Cross-framework tests ensure:

1. **API Parity** - Same props and events across frameworks
2. **Feature Parity** - Same functionality across frameworks
3. **Theme Parity** - Same visual appearance across frameworks
4. **Dependency Parity** - Equivalent dependencies across frameworks

### Integration Tests

Integration tests validate:

1. **Registry Integration** - Components are properly registered
2. **Build Integration** - Components compile correctly
3. **Documentation Integration** - Documentation is generated correctly
4. **Example Integration** - Examples work correctly

## Running Tests

### Individual Component Tests

```bash
# Test Yew radio-group
cargo test -p shadcn-ui-yew-radio-group

# Test Leptos radio-group
cargo test -p shadcn-ui-leptos-radio-group
```

### Integration Tests

```bash
# Run all integration tests
cargo test --test radio_group_integration_test

# Run specific integration test
cargo test test_radio_group_cross_framework_parity
```

### Complete Test Suite

```bash
# Run the comprehensive test script
./scripts/test_radio_group.sh
```

### Test Script Features

The test script (`scripts/test_radio_group.sh`) provides:

- **Colored Output** - Easy-to-read test results
- **Comprehensive Coverage** - All test types in one command
- **Detailed Reporting** - Pass/fail counts and summaries
- **Error Handling** - Proper exit codes for CI/CD

## Test Results

### Success Output

```
🧪 Running RadioGroup Component Tests
=====================================
ℹ️  INFO: Running Yew RadioGroup Unit Tests...
✅ PASS: Yew RadioGroup Unit Tests completed successfully
ℹ️  INFO: Running Leptos RadioGroup Unit Tests...
✅ PASS: Leptos RadioGroup Unit Tests completed successfully
...

📊 Test Summary
===============
Total tests: 12
Passed: 12
Failed: 0
✅ PASS: All RadioGroup tests passed! 🎉
```

### Failure Output

```
❌ FAIL: Yew RadioGroup Unit Tests failed
...

📊 Test Summary
===============
Total tests: 12
Passed: 8
Failed: 4
❌ FAIL: 4 test(s) failed. Please check the output above.
```

## Best Practices

### Writing Tests

1. **Test Structure** - Follow the established patterns
2. **Naming** - Use descriptive test names
3. **Assertions** - Include meaningful assertion messages
4. **Coverage** - Test all public APIs and edge cases
5. **Isolation** - Tests should be independent

### Framework-Specific Considerations

#### Yew Tests

- Use `wasm_bindgen_test` for browser tests
- Use `yew::start_app_with_props` for component testing
- Test DOM queries and event handling

#### Leptos Tests

- Use `mount_to_body` for component mounting
- Use signals for state management testing
- Test context providers and consumers

### Cross-Framework Testing

1. **API Consistency** - Ensure same props and events
2. **Feature Completeness** - All features available in all frameworks
3. **Theme Support** - Both default and New York variants
4. **Accessibility** - Same ARIA attributes and behavior

## Continuous Integration

### GitHub Actions

The testing infrastructure integrates with GitHub Actions:

```yaml
- name: Run RadioGroup Tests
  run: ./scripts/test_radio_group.sh
```

### Local Development

For local development:

```bash
# Quick test during development
cargo test -p shadcn-ui-yew-radio-group --lib

# Full test suite
./scripts/test_radio_group.sh

# Specific test category
cargo test test_radio_group_cross_framework_parity
```

## Future Enhancements

### Planned Features

1. **Visual Regression Testing** - Automated visual comparison
2. **Performance Testing** - Bundle size and runtime performance
3. **Browser Testing** - Cross-browser compatibility
4. **Mobile Testing** - Touch interactions and responsive design
5. **Accessibility Auditing** - Automated accessibility scanning

### Test Coverage Metrics

- **Line Coverage** - Track code coverage percentage
- **Branch Coverage** - Ensure all code paths are tested
- **Function Coverage** - All public functions tested
- **Integration Coverage** - All framework combinations tested

## Troubleshooting

### Common Issues

1. **WASM Tests Failing** - Ensure `wasm32-unknown-unknown` target is installed
2. **Browser Tests** - Ensure browser environment is available
3. **Dependency Issues** - Check Cargo.toml dependencies
4. **Build Failures** - Verify component implementations

### Debugging Tips

1. **Verbose Output** - Use `RUST_LOG=debug` for detailed logging
2. **Single Test** - Run individual tests for focused debugging
3. **Browser Console** - Check browser console for WASM test errors
4. **Test Isolation** - Ensure tests don't interfere with each other

## Conclusion

The testing infrastructure provides comprehensive coverage for component quality, cross-framework parity, and maintainability. By following the established patterns and running the complete test suite, developers can ensure that components work correctly across all supported frameworks and maintain the high quality standards of the Rust shadcn/ui project.
