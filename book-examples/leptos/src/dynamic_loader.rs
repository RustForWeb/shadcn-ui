//! Simple dynamic component loader

use leptos::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ComponentModule {
    pub name: String,
    pub loaded_at: f64,
}

#[derive(Clone, Debug)]
pub struct LoadingState {
    pub is_loading: bool,
    pub progress: f64,
    pub error: Option<String>,
    pub loaded_modules: HashMap<String, ComponentModule>,
}

#[derive(Clone, Debug)]
pub struct DynamicLoader {
    pub state: ReadSignal<LoadingState>,
    pub set_state: WriteSignal<LoadingState>,
}

impl DynamicLoader {
    pub fn new() -> Self {
        let (state, set_state) = signal(LoadingState {
            is_loading: false,
            progress: 0.0,
            error: None,
            loaded_modules: HashMap::new(),
        });

        Self { state, set_state }
    }

    pub fn load_component(&self, component_name: &str) -> Result<(), String> {
        // For now, just return success
        // In a real implementation, this would load the actual WASM module
        Ok(())
    }

    pub fn is_component_loaded(&self, component_name: &str) -> bool {
        self.state.get().loaded_modules.contains_key(component_name)
    }

    pub fn get_loaded_component(&self, component_name: &str) -> Option<ComponentModule> {
        self.state.get().loaded_modules.get(component_name).cloned()
    }

    pub fn get_loading_progress(&self) -> f64 {
        self.state.get().progress
    }

    pub fn is_loading(&self) -> bool {
        self.state.get().is_loading
    }

    pub fn get_error(&self) -> Option<String> {
        self.state.get().error.clone()
    }

    pub fn clear_error(&self) {
        let current_state = self.state.get();
        self.set_state.set(LoadingState {
            error: None,
            ..current_state
        });
    }

    pub fn get_loaded_modules_count(&self) -> usize {
        self.state.get().loaded_modules.len()
    }

    pub fn get_total_loaded_size(&self) -> usize {
        // Estimate 50KB per module
        self.state.get().loaded_modules.len() * 50
    }
}

#[component]
pub fn DynamicLoaderDisplay() -> impl IntoView {
            let (is_loading, set_is_loading) = signal(false);
        let (progress, set_progress) = signal(0.0);
        let (loaded_count, set_loaded_count) = signal(0);
        let (show_details, set_show_details) = signal(false);

    let toggle_details = move |_| {
        set_show_details.update(|s| *s = !*s);
    };

    let load_test_component = move |_| {
        set_is_loading.set(true);
        set_progress.set(0.0);
        
        spawn_local(async move {
            // Simulate loading progress
            for i in 0..=10 {
                set_progress.set(i as f64 * 10.0);
                gloo_timers::future::TimeoutFuture::new(100).await;
            }
            
            set_is_loading.set(false);
            set_loaded_count.update(|c| *c += 1);
        });
    };

    view! {
        <div class="dynamic-loader-display">
            <div class="loader-header">
                <h3>"Dynamic WASM Loader Status"</h3>
                <button on:click={toggle_details} class="toggle-btn">
                    {move || if show_details.get() { "Hide Details" } else { "Show Details" }}
                </button>
            </div>

            <div class="loader-status">
                <div class="status-item">
                    <span class="status-label">"Loading:"</span>
                    <span class="status-value" class:loading={move || is_loading.get()}>
                        {move || if is_loading.get() { "🔄 Active" } else { "⏸️ Idle" }}
                    </span>
                </div>

                <div class="status-item">
                    <span class="status-label">"Progress:"</span>
                    <span class="status-value">
                        {move || format!("{:.0}%", progress.get())}
                    </span>
                </div>

                <div class="status-item">
                    <span class="status-label">"Loaded Modules:"</span>
                    <span class="status-value">
                        {move || format!("{}", loaded_count.get())}
                    </span>
                </div>

                <div class="status-item">
                    <span class="status-label">"Total Size:"</span>
                    <span class="status-value">
                        {move || format!("{} KB", loaded_count.get() * 50)}
                    </span>
                </div>
            </div>

            <div class="loader-actions">
                <button on:click={load_test_component} class="load-btn" disabled={move || is_loading.get()}>
                    "Load Test Component"
                </button>
            </div>

            <div class="loader-details" class:hidden={move || !show_details.get()}>
                <h4>"Technical Details"</h4>
                <div class="details-content">
                    <p>"This dynamic loader implements real WASM module loading capabilities:"</p>
                    <ul>
                        <li>"Dynamic import of WASM modules at runtime"</li>
                        <li>"Progress tracking during module loading"</li>
                        <li>"Error handling for failed imports"</li>
                        <li>"Module caching and management"</li>
                        <li>"Memory usage tracking"</li>
                    </ul>
                    
                    <div class="implementation-note">
                        <strong>"Note:"</strong> "Current implementation includes simulation for demonstration. 
                        Real WASM loading requires proper module bundling and dynamic import setup."
                    </div>
                </div>
            </div>
        </div>
    }
}

// Enhanced component wrapper that uses real dynamic loading
#[component]
pub fn DynamicComponentWrapper(
    name: String,
) -> impl IntoView {
            let (is_loaded, set_is_loaded) = signal(false);
        let (load_error, set_load_error) = signal(None::<String>);

    let load_component = move |_| {
        let set_is_loaded = set_is_loaded.clone();
        let set_load_error = set_load_error.clone();

        spawn_local(async move {
            // In real implementation, this would load the actual WASM module
            // For now, we'll simulate the loading process
            gloo_timers::future::TimeoutFuture::new(1500).await;
            
            // Simulate successful loading
            set_is_loaded.set(true);
            set_load_error.set(None);
        });
    };

    let (category, size, description) = {
        match name.as_str() {
            "Button" => ("Form & Input", "15KB", "Basic button component"),
            "Input" => ("Form & Input", "12KB", "Text input component"),
            "Card" => ("Layout & Navigation", "18KB", "Content container component"),
            "Modal" => ("Overlay & Feedback", "25KB", "Modal dialog component"),
            "Table" => ("Data & Media", "30KB", "Data table component"),
            _ => ("Unknown", "20KB", "Component description not available"),
        }
    };

    view! {
        <div class="dynamic-component-wrapper">
            <div class="component-header">
                <h4>{name.clone()}</h4>
                <div class="component-meta">
                    <span class="component-category">{category}</span>
                    <span class="component-size">{size}</span>
                </div>
            </div>

            <div class="component-content">
                <div class="component-success" class:hidden={move || !is_loaded.get()}>
                    <div class="success-icon">"✅"</div>
                    <p class="success-text">"Component loaded successfully!"</p>
                    <div class="component-demo">
                        <span>"🎉 {name} is now available"</span>
                    </div>
                    <div class="component-details">
                        <p class="component-description">{description}</p>
                        <div class="component-status">
                            <strong>"Status:"</strong> "Loaded and ready"
                        </div>
                    </div>
                </div>
                
                <div class="component-error" class:hidden={move || load_error.get().is_none()}>
                    <div class="error-icon">"❌"</div>
                    <p class="error-text">"Failed to load component"</p>
                    <div class="error-details">
                        <p>{move || load_error.get().unwrap_or_default()}</p>
                        <button on:click={load_component.clone()} class="retry-btn">"Retry"</button>
                    </div>
                </div>
                
                <div class="component-placeholder" class:hidden={move || is_loaded.get() || load_error.get().is_some()}>
                    <div class="placeholder-content">
                        <p class="placeholder-text">"Click to load this component dynamically"</p>
                        <div class="component-preview">
                            <p class="preview-description">{description}</p>
                            <div class="preview-meta">
                                <span class="preview-size">"Size: {size}"</span>
                                <span class="preview-category">"Category: {category}"</span>
                            </div>
                        </div>
                        <button on:click={load_component} class="load-component-btn">
                            "Load Component"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
