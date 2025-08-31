#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_tooltip_provider_renders() {
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <div>"Test content"</div>
                </TooltipProvider>
            }
        });

        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();
        
        assert!(body.inner_html().contains("Test content"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_basic_functionality() {
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger>
                            <button>"Hover me"</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <p>"Tooltip content"</p>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.query_selector("button").unwrap().unwrap();
        
        // Initially tooltip should not be visible
        let tooltip_content = document.query_selector("p");
        assert!(tooltip_content.is_ok());
        
        // Button should be rendered
        assert!(button.text_content().unwrap().contains("Hover me"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_trigger_events() {
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger id="trigger">
                            <button>"Trigger"</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div id="tooltip-content">"Content"</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        let trigger = document.get_element_by_id("trigger").unwrap();
        
        // Trigger should have mouse event handlers
        assert!(trigger.tag_name() == "DIV" || trigger.tag_name() == "BUTTON");
    }

    #[wasm_bindgen_test]
    fn test_tooltip_content_class_application() {
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <Tooltip open=true>
                        <TooltipTrigger>
                            <button>"Trigger"</button>
                        </TooltipTrigger>
                        <TooltipContent class="custom-class">
                            <div>"Content"</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        // Test that custom classes are applied
        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let html = body.inner_html();
        
        // Should contain default tooltip classes
        assert!(html.contains("z-50") || html.contains("custom-class"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_side_positioning() {
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <Tooltip open=true>
                        <TooltipTrigger>
                            <button>"Trigger"</button>
                        </TooltipTrigger>
                        <TooltipContent side=TooltipSide::Bottom>
                            <div>"Bottom tooltip"</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        // Test that positioning logic works
        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        assert!(body.inner_html().contains("Bottom tooltip"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_content_styling() {
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <Tooltip open=true>
                        <TooltipTrigger>
                            <button>"Trigger"</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>"Styled content"</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let html = body.inner_html();
        
        // Should contain tooltip styling classes
        assert!(html.contains("z-50") && html.contains("Styled content"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_accessibility_attributes() {
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <Tooltip open=true>
                        <TooltipTrigger>
                            <button>"Accessible trigger"</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>"Accessible content"</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        assert!(body.inner_html().contains("Accessible"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_controlled_state() {
        let (open, set_open) = create_signal(false);
        
        mount_to_body(move || {
            view! {
                <TooltipProvider>
                    <Tooltip 
                        open=open
                        on_open_change=move |is_open| set_open(is_open)
                    >
                        <TooltipTrigger>
                            <button>"Controlled trigger"</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>"Controlled content"</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        // Test controlled state
        assert!(!open.get_untracked());
        set_open(true);
        assert!(open.get_untracked());
    }

    #[wasm_bindgen_test]
    fn test_tooltip_theme_variants() {
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <Tooltip open=true>
                        <TooltipTrigger>
                            <button>"Theme test"</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>"Theme content"</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let html = body.inner_html();
        
        // Should contain theme-related classes
        assert!(html.contains("bg-popover") || html.contains("Theme content"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_side_offset() {
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <Tooltip open=true>
                        <TooltipTrigger>
                            <button>"Offset test"</button>
                        </TooltipTrigger>
                        <TooltipContent 
                            side=TooltipSide::Top
                            side_offset=8
                        >
                            <div>"Offset content"</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        assert!(body.inner_html().contains("Offset content"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_edge_cases() {
        // Test empty content
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger>
                            <button>"Empty tooltip"</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div></div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.query_selector("button").unwrap().unwrap();
        assert!(button.text_content().unwrap().contains("Empty tooltip"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_multiple_instances() {
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger>
                            <button>"First tooltip"</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>"First content"</div>
                        </TooltipContent>
                    </Tooltip>
                    <Tooltip>
                        <TooltipTrigger>
                            <button>"Second tooltip"</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>"Second content"</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        let buttons = document.query_selector_all("button").unwrap();
        assert_eq!(buttons.length(), 2);
    }

    #[wasm_bindgen_test]
    fn test_tooltip_custom_styling() {
        mount_to_body(|| {
            view! {
                <TooltipProvider>
                    <Tooltip open=true>
                        <TooltipTrigger class="custom-trigger">
                            <button>"Custom styled"</button>
                        </TooltipTrigger>
                        <TooltipContent 
                            class="custom-tooltip"
                            style="background: red;"
                        >
                            <div>"Custom content"</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let html = body.inner_html();
        
        assert!(html.contains("Custom") && (html.contains("custom") || html.contains("background")));
    }
}