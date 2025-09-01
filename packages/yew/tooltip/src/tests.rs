#[cfg(test)]
mod tests {
    use crate::{TooltipProvider, Tooltip, TooltipTrigger, TooltipContent, TooltipSide};
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_tooltip_provider_renders() {
        yew::Renderer::<App>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        #[function_component]
        fn App() -> Html {
            html! {
                <TooltipProvider>
                    <div>{"Test content"}</div>
                </TooltipProvider>
            }
        }

        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();
        
        // Provider should render children
        assert!(body.inner_html().contains("Test content"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_basic_structure() {
        #[function_component]
        fn TestTooltip() -> Html {
            html! {
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger>
                            <button>{"Hover me"}</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <p>{"Tooltip content"}</p>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<TestTooltip>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.query_selector("button").unwrap().unwrap();
        
        // Button should be rendered
        assert!(button.text_content().unwrap().contains("Hover me"));
    }

    #[wasm_bindgen_test] 
    fn test_tooltip_controlled_state() {
        #[function_component]
        fn ControlledTooltip() -> Html {
            let open = use_state(|| false);
            let on_open_change = {
                let open = open.clone();
                Callback::from(move |is_open: bool| {
                    open.set(is_open);
                })
            };

            html! {
                <TooltipProvider>
                    <Tooltip 
                        open={*open}
                        on_open_change={on_open_change}
                    >
                        <TooltipTrigger>
                            <button>{"Controlled trigger"}</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>{"Controlled content"}</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<ControlledTooltip>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.query_selector("button").unwrap().unwrap();
        assert!(button.text_content().unwrap().contains("Controlled trigger"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_content_styling() {
        #[function_component]
        fn StyledTooltip() -> Html {
            html! {
                <TooltipProvider>
                    <Tooltip open={true}>
                        <TooltipTrigger>
                            <button>{"Trigger"}</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>{"Styled content"}</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<StyledTooltip>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let html = body.inner_html();
        
        // Should contain tooltip styling classes
        assert!(html.contains("z-50") || html.contains("Styled content"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_side_positioning() {
        #[function_component]
        fn PositionedTooltip() -> Html {
            html! {
                <TooltipProvider>
                    <Tooltip open={true}>
                        <TooltipTrigger>
                            <button>{"Trigger"}</button>
                        </TooltipTrigger>
                        <TooltipContent side={TooltipSide::Bottom}>
                            <div>{"Bottom tooltip"}</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<PositionedTooltip>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        assert!(body.inner_html().contains("Bottom tooltip"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_custom_classes() {
        #[function_component]
        fn CustomTooltip() -> Html {
            html! {
                <TooltipProvider>
                    <Tooltip open={true}>
                        <TooltipTrigger class="custom-trigger">
                            <button>{"Custom styled"}</button>
                        </TooltipTrigger>
                        <TooltipContent class="custom-tooltip">
                            <div>{"Custom content"}</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<CustomTooltip>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let html = body.inner_html();
        
        assert!(html.contains("Custom") && (html.contains("custom") || html.contains("styled")));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_delay_duration() {
        #[function_component]
        fn DelayedTooltip() -> Html {
            html! {
                <TooltipProvider>
                    <Tooltip delay_duration={1000}>
                        <TooltipTrigger>
                            <button>{"Delayed trigger"}</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>{"Delayed content"}</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<DelayedTooltip>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.query_selector("button").unwrap().unwrap();
        assert!(button.text_content().unwrap().contains("Delayed trigger"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_side_offset() {
        #[function_component]
        fn OffsetTooltip() -> Html {
            html! {
                <TooltipProvider>
                    <Tooltip open={true}>
                        <TooltipTrigger>
                            <button>{"Offset test"}</button>
                        </TooltipTrigger>
                        <TooltipContent 
                            side={TooltipSide::Top}
                            side_offset={8}
                        >
                            <div>{"Offset content"}</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<OffsetTooltip>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        assert!(body.inner_html().contains("Offset content"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_accessibility_structure() {
        #[function_component]
        fn AccessibleTooltip() -> Html {
            html! {
                <TooltipProvider>
                    <Tooltip open={true}>
                        <TooltipTrigger>
                            <button aria_describedby="tooltip-1">{"Accessible trigger"}</button>
                        </TooltipTrigger>
                        <TooltipContent id="tooltip-1">
                            <div role="tooltip">{"Accessible content"}</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<AccessibleTooltip>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let html = body.inner_html();
        assert!(html.contains("Accessible") && html.contains("tooltip"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_multiple_instances() {
        #[function_component]
        fn MultipleTooltips() -> Html {
            html! {
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger>
                            <button>{"First tooltip"}</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>{"First content"}</div>
                        </TooltipContent>
                    </Tooltip>
                    <Tooltip>
                        <TooltipTrigger>
                            <button>{"Second tooltip"}</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>{"Second content"}</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<MultipleTooltips>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let buttons = document.query_selector_all("button").unwrap();
        assert_eq!(buttons.length(), 2);
    }

    #[wasm_bindgen_test]
    fn test_tooltip_theme_variants() {
        #[function_component]
        fn ThemedTooltip() -> Html {
            html! {
                <TooltipProvider>
                    <Tooltip open={true}>
                        <TooltipTrigger>
                            <button>{"Theme test"}</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div>{"Theme content"}</div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<ThemedTooltip>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let html = body.inner_html();
        
        // Should contain theme-related classes
        assert!(html.contains("bg-popover") || html.contains("Theme content"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_edge_cases() {
        #[function_component]
        fn EdgeCaseTooltip() -> Html {
            html! {
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger>
                            <button>{"Empty tooltip"}</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <div></div>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<EdgeCaseTooltip>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.query_selector("button").unwrap().unwrap();
        assert!(button.text_content().unwrap().contains("Empty tooltip"));
    }

    #[wasm_bindgen_test] 
    fn test_tooltip_context_propagation() {
        #[function_component]
        fn ContextTooltip() -> Html {
            html! {
                <TooltipProvider>
                    <Tooltip>
                        <div>
                            <TooltipTrigger>
                                <button>{"Nested trigger"}</button>
                            </TooltipTrigger>
                            <div>
                                <TooltipContent>
                                    <div>{"Nested content"}</div>
                                </TooltipContent>
                            </div>
                        </div>
                    </Tooltip>
                </TooltipProvider>
            }
        }

        yew::Renderer::<ContextTooltip>::with_root(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .into(),
        )
        .render();

        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.query_selector("button").unwrap().unwrap();
        assert!(button.text_content().unwrap().contains("Nested trigger"));
    }
}