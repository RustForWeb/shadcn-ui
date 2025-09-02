//! Lazy loading system for shadcn/ui Leptos components
//! 
//! This module provides lazy loading capabilities to reduce initial bundle size
//! by loading components only when they're needed.

use leptos::prelude::*;
use leptos::html::ElementChild;
use leptos::task::spawn_local;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;



/// Lazy component loader that manages dynamic imports
#[derive(Clone)]
pub struct LazyComponentLoader {
    components: Arc<Mutex<HashMap<String, ComponentLoader>>>,
}

/// Component loader function type
pub type ComponentLoader = Box<dyn Fn() -> Result<View<()>, String> + Send + Sync>;

impl LazyComponentLoader {
    /// Create a new lazy component loader
    pub fn new() -> Self {
        Self {
            components: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a component for lazy loading
    pub fn register_component<F>(&self, name: &str, loader: F)
    where
        F: Fn() -> Result<View<()>, String> + Send + Sync + 'static,
    {
        let mut components = self.components.lock().unwrap();
        components.insert(name.to_string(), Box::new(loader));
    }

    /// Load a component by name
    pub fn load_component(&self, name: &str) -> Result<View<()>, String> {
        let components = self.components.lock().unwrap();
        if let Some(loader) = components.get(name) {
            loader()
        } else {
            Err(format!("Component '{}' not found", name))
        }
    }

    /// Check if a component is registered
    pub fn has_component(&self, name: &str) -> bool {
        let components = self.components.lock().unwrap();
        components.contains_key(name)
    }

    /// Get all registered component names
    pub fn registered_components(&self) -> Vec<String> {
        let components = self.components.lock().unwrap();
        components.keys().cloned().collect()
    }
}

impl Default for LazyComponentLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Lazy component wrapper that loads components on demand
#[component]
pub fn LazyComponent(
    #[prop(into)] name: String,
    #[prop(optional)] fallback: Option<View<()>>,
    #[prop(optional)] error_fallback: Option<Box<dyn Fn(String) -> View<()> + Send + Sync>>,
) -> impl IntoView {
    let loader = use_context::<LazyComponentLoader>()
        .expect("LazyComponentLoader not found in context");
    
    let (component, set_component) = signal(None::<Result<View<()>, String>>);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    // Load component when name changes
    Effect::new(move |_| {
        let name = name.clone();
        let loader = loader.clone();
        
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            // Simulate async loading
            let result = loader.load_component(&name);
            
            set_component.set(Some(result.clone()));
            set_loading.set(false);
            
            if let Err(err) = result {
                set_error.set(Some(err));
            }
        });
    });

    // Render based on state
    move || {
        if loading.get() {
            // Show fallback while loading
            fallback.clone().unwrap_or_else(|| {
                let view: View<()> = view! {
                    <div class="lazy-loading-fallback">
                        <div class="loading-spinner"></div>
                        <p>"Loading component..."</p>
                    </div>
                };
                view
            })
        } else if let Some(Ok(comp)) = component.get() {
            // Component loaded successfully
            comp
        } else if let Some(err) = error.get() {
            // Component failed to load
            if let Some(error_fn) = &error_fallback {
                error_fn(err)
            } else {
                let view: View<()> = view! {
                    <div class="lazy-loading-error">
                        <p class="error-message">"Failed to load component: {err}"</p>
                        <button 
                            class="retry-button"
                            on:click=move |_| {
                                set_loading.set(true);
                                set_error.set(None);
                            }
                        >
                            "Retry"
                        </button>
                    </div>
                };
                view
            }
        } else {
            // No component loaded yet
            let view: View<()> = view! { <div></div> };
            view
        }
    }
}

/// Hook for lazy loading components
pub fn use_lazy_component(name: &str) -> (ReadSignal<bool>, ReadSignal<Option<Result<View<()>, String>>>, WriteSignal<bool>) {
    let (loading, set_loading) = signal(false);
    let (component, set_component) = signal(None::<Result<View<()>, String>>);
    
    let loader = use_context::<LazyComponentLoader>()
        .expect("LazyComponentLoader not found in context");
    
    let name = name.to_string();
    let load = move || {
        set_loading.set(true);
        
        spawn_local(async move {
            let result = loader.load_component(&name);
            set_component.set(Some(result));
            set_loading.set(false);
        });
    };
    
    (loading, component, set_loading)
}

/// Component bundle analyzer for optimization
pub struct BundleAnalyzer;

impl BundleAnalyzer {
    /// Analyze component usage and provide optimization suggestions
    pub fn analyze_usage(components: &[String]) -> BundleAnalysis {
        let mut analysis = BundleAnalysis::new();
        
        // Analyze component categories
        let form_components = ["input", "label", "checkbox", "radio-group", "select", "textarea", "form"];
        let layout_components = ["card", "separator", "skeleton", "tabs"];
        let interactive_components = ["button", "checkbox", "radio-group", "select", "switch", "tabs"];
        
        let form_count = components.iter().filter(|c| form_components.contains(&c.as_str())).count();
        let layout_count = components.iter().filter(|c| layout_components.contains(&c.as_str())).count();
        let interactive_count = components.iter().filter(|c| interactive_components.contains(&c.as_str())).count();
        
        analysis.form_components = form_count;
        analysis.layout_components = layout_count;
        analysis.interactive_components = interactive_count;
        analysis.total_components = components.len();
        
        // Generate recommendations
        if form_count > 4 {
            analysis.recommendations.push("Consider lazy loading form components to reduce initial bundle size".to_string());
        }
        
        if layout_count > 3 {
            analysis.recommendations.push("Layout components can be loaded on demand for better performance".to_string());
        }
        
        if interactive_count > 5 {
            analysis.recommendations.push("Interactive components benefit from lazy loading for better UX".to_string());
        }
        
        analysis
    }
}

/// Bundle analysis results
#[derive(Debug, Clone)]
pub struct BundleAnalysis {
    pub form_components: usize,
    pub layout_components: usize,
    pub interactive_components: usize,
    pub total_components: usize,
    pub recommendations: Vec<String>,
}

impl BundleAnalysis {
    fn new() -> Self {
        Self {
            form_components: 0,
            layout_components: 0,
            interactive_components: 0,
            total_components: 0,
            recommendations: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_component_loader() {
        let loader = LazyComponentLoader::new();
        
        // Register a test component
        loader.register_component("test", || {
            Ok(view! { <div>"Test Component"</div> })
        });
        
        assert!(loader.has_component("test"));
        assert!(!loader.has_component("nonexistent"));
        
        let components = loader.registered_components();
        assert!(components.contains(&"test".to_string()));
    }

    #[test]
    fn test_bundle_analyzer() {
        let components = vec!["button".to_string(), "input".to_string(), "card".to_string()];
        let analysis = BundleAnalyzer::analyze_usage(&components);
        
        assert_eq!(analysis.total_components, 3);
        assert_eq!(analysis.form_components, 1);
        assert_eq!(analysis.layout_components, 1);
        assert_eq!(analysis.interactive_components, 2);
    }
}
