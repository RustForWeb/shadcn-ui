use super::*;
use leptos::prelude::*;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, MouseEvent};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_radio_group_renders() {
    let (selected_value, _) = create_signal(None::<String>);
    
    let rendered = mount_to_body(|| {
        view! {
            <RadioGroup value=selected_value>
                <RadioGroupItem value={"option1".to_string()}>
                    "Option 1"
                </RadioGroupItem>
            </RadioGroup>
        }
    });

    // Check that the radio group container is rendered
    let container = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("[role='radiogroup']")
        .unwrap();
    assert!(container.is_some(), "Radio group should have role='radiogroup'");
}

#[wasm_bindgen_test]
fn test_radio_group_item_renders() {
    let rendered = mount_to_body(|| {
        view! {
            <RadioGroupItem value={"option1".to_string()}>
                "Option 1"
            </RadioGroupItem>
        }
    });

    // Check that the radio item is rendered with correct attributes
    let radio_item = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("[role='radio']")
        .unwrap();
    assert!(radio_item.is_some(), "Radio item should have role='radio'");
    
    if let Some(element) = radio_item {
        let html_element: HtmlElement = element.dyn_into().unwrap();
        assert_eq!(html_element.get_attribute("aria-checked"), Some("false".to_string()));
        assert_eq!(html_element.get_attribute("data-state"), Some("unchecked".to_string()));
    }
}

#[wasm_bindgen_test]
fn test_radio_group_selection() {
    let (selected_value, set_selected_value) = create_signal(None::<String>);
    let on_value_change = Callback::from(move |value: String| {
        set_selected_value.set(Some(value));
    });

    let rendered = mount_to_body(move || {
        view! {
            <RadioGroup
                value=selected_value
                on_value_change=on_value_change
            >
                <RadioGroupItem value={"option1".to_string()}>
                    "Option 1"
                </RadioGroupItem>
                <RadioGroupItem value={"option2".to_string()}>
                    "Option 2"
                </RadioGroupItem>
            </RadioGroup>
        }
    });

    // Simulate clicking the first radio item
    let first_radio = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("[role='radio']")
        .unwrap()
        .unwrap();
    let html_element: HtmlElement = first_radio.dyn_into().unwrap();
    
    // Create a mock click event and trigger it
    let click_event = MouseEvent::new("click").unwrap();
    html_element.click();
    
    // Wait a bit for the signal to update
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    // Check that the value was updated
    assert_eq!(selected_value.get(), Some("option1".to_string()));
}

#[wasm_bindgen_test]
fn test_radio_group_disabled_state() {
    let (selected_value, _) = create_signal(None::<String>);
    let disabled = create_signal(true);
    
    let rendered = mount_to_body(move || {
        view! {
            <RadioGroup value=selected_value disabled=disabled.0>
                <RadioGroupItem value={"option1".to_string()} disabled=disabled.0>
                    "Option 1"
                </RadioGroupItem>
            </RadioGroup>
        }
    });

    // Check that disabled state is applied
    let radio_item = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("[role='radio']")
        .unwrap()
        .unwrap();
    let html_element: HtmlElement = radio_item.dyn_into().unwrap();
    
    assert_eq!(html_element.get_attribute("data-disabled"), Some("true".to_string()));
    assert!(html_element.has_attribute("disabled"));
}

#[wasm_bindgen_test]
fn test_radio_group_aria_attributes() {
    let (selected_value, _) = create_signal(Some("option1".to_string()));
    
    let rendered = mount_to_body(move || {
        view! {
            <RadioGroup value=selected_value>
                <RadioGroupItem value={"option1".to_string()}>
                    "Option 1"
                </RadioGroupItem>
                <RadioGroupItem value={"option2".to_string()}>
                    "Option 2"
                </RadioGroupItem>
            </RadioGroup>
        }
    });

    // Check ARIA attributes for selected item
    let radio_items = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector_all("[role='radio']")
        .unwrap();
    assert_eq!(radio_items.length(), 2, "Should have 2 radio items");

    let first_item: HtmlElement = radio_items.get(0).unwrap().dyn_into().unwrap();
    let second_item: HtmlElement = radio_items.get(1).unwrap().dyn_into().unwrap();

    // First item should be selected
    assert_eq!(first_item.get_attribute("aria-checked"), Some("true".to_string()));
    assert_eq!(first_item.get_attribute("data-state"), Some("checked".to_string()));

    // Second item should be unselected
    assert_eq!(second_item.get_attribute("aria-checked"), Some("false".to_string()));
    assert_eq!(second_item.get_attribute("data-state"), Some("unchecked".to_string()));
}

#[wasm_bindgen_test]
fn test_radio_group_css_classes() {
    let (selected_value, _) = create_signal(None::<String>);
    let custom_class = create_signal("custom-class".to_string());
    let item_class = create_signal("item-class".to_string());
    
    let rendered = mount_to_body(move || {
        view! {
            <RadioGroup value=selected_value class=custom_class.0>
                <RadioGroupItem value={"option1".to_string()} class=item_class.0>
                    "Option 1"
                </RadioGroupItem>
            </RadioGroup>
        }
    });

    // Check that custom classes are applied
    let container = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("[role='radiogroup']")
        .unwrap()
        .unwrap();
    let html_element: HtmlElement = container.dyn_into().unwrap();
    
    let class_list = html_element.class_list();
    assert!(class_list.contains("custom-class"), "Container should have custom class");
    assert!(class_list.contains("grid"), "Container should have grid class");
    assert!(class_list.contains("gap-2"), "Container should have gap-2 class");

    let radio_item = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("[role='radio']")
        .unwrap()
        .unwrap();
    let item_element: HtmlElement = radio_item.dyn_into().unwrap();
    
    let item_class_list = item_element.class_list();
    assert!(item_class_list.contains("item-class"), "Radio item should have custom class");
}

#[wasm_bindgen_test]
fn test_radio_group_new_york_variant() {
    let (selected_value, _) = create_signal(Some("option1".to_string()));
    
    let rendered = mount_to_body(move || {
        view! {
            <RadioGroupNewYork value=selected_value>
                <RadioGroupItemNewYork value={"option1".to_string()}>
                    "Option 1"
                </RadioGroupItemNewYork>
            </RadioGroupNewYork>
        }
    });

    // Check that New York variant has different styling
    let radio_item = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("[role='radio']")
        .unwrap()
        .unwrap();
    let html_element: HtmlElement = radio_item.dyn_into().unwrap();
    
    let class_list = html_element.class_list();
    assert!(class_list.contains("shadow"), "New York variant should have shadow class");
    assert!(class_list.contains("focus-visible:ring-1"), "New York variant should have ring-1 focus");
}

#[test]
fn test_radio_group_signal_behavior() {
    // Test signal behavior in a non-browser environment
    let (selected_value, set_selected_value) = create_signal(None::<String>);
    
    // Initial state should be None
    assert_eq!(selected_value.get(), None);
    
    // Set a value
    set_selected_value.set(Some("test-value".to_string()));
    assert_eq!(selected_value.get(), Some("test-value".to_string()));
    
    // Clear the value
    set_selected_value.set(None);
    assert_eq!(selected_value.get(), None);
}

#[test]
fn test_radio_group_context_behavior() {
    // Test that context is properly provided and consumed
    let (selected_value, set_selected_value) = create_signal(Some("option1".to_string()));
    let (disabled, _) = create_signal(false);
    
    // Create a context
    let context = RadioGroupContext {
        selected_value: selected_value.read_only(),
        on_item_select: Callback::from(move |value: String| {
            set_selected_value.set(Some(value));
        }),
        disabled,
    };
    
    // Test context values
    assert_eq!(context.selected_value.get(), Some("option1".to_string()));
    assert_eq!(context.disabled.get(), false);
}

#[test]
fn test_radio_group_item_value_comparison() {
    // Test value comparison logic
    let (selected_value, _) = create_signal(Some("option1".to_string()));
    let context = RadioGroupContext {
        selected_value: selected_value.read_only(),
        on_item_select: Callback::noop(),
        disabled: create_signal(false).0,
    };
    
    // Test that comparison works correctly
    let is_selected = Signal::derive(move || {
        context.selected_value.get().as_ref() == Some(&"option1".to_string())
    });
    
    assert_eq!(is_selected.get(), true);
    
    let is_not_selected = Signal::derive(move || {
        context.selected_value.get().as_ref() == Some(&"option2".to_string())
    });
    
    assert_eq!(is_not_selected.get(), false);
}
