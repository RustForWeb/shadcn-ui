use leptos::*;
use leptos::prelude::*;
use crate::lazy_loading::LazyComponentWrapper;
use crate::bundle_analyzer::BundleAnalysisDisplay;
use crate::dynamic_loader::BundleStatusDisplay;

#[component]
pub fn App() -> impl IntoView {
    let (current_theme, set_current_theme) = signal("default".to_string());

    let toggle_theme = move |_| {
        let new_theme = if current_theme.get() == "default" { "new_york" } else { "default" };
        set_current_theme.set(new_theme.to_string());
    };

    view! {
        <div class="app-container">
            <header class="app-header">
                <h1>"🚀 ShadCN UI - Leptos Bundle Optimization Demo"</h1>
                <p>"Dynamic Lazy Loading with Essential Components"</p>
            </header>

            <main class="main-content">
                <div class="optimization-panels">
                    <div class="panel bundle-analysis">
                        <BundleAnalysisDisplay />
                    </div>
                    
                    <div class="panel bundle-status">
                        <BundleStatusDisplay />
                    </div>
                </div>

                <div class="component-showcase">
                    <div class="showcase-header">
                        <h2>"Component Showcase"</h2>
                        <div class="theme-toggle">
                            <button on:click={toggle_theme} class="theme-btn">
                                {move || format!("Switch to {} Theme", 
                                    if current_theme.get() == "default" { "New York" } else { "Default" }
                                )}
                            </button>
                        </div>
                    </div>

                    <div class="essential-components">
                        <h3>"Essential Components (Always Loaded)"</h3>
                        <div class="component-grid">
                            <div class="component-item">
                                <h4>"Button"</h4>
                                <button class="component-btn">"Button Component"</button>
                            </div>
                            <div class="component-item">
                                <h4>"Input"</h4>
                                <input class="component-input" placeholder="Input Component" />
                            </div>
                            <div class="component-item">
                                <h4>"Label"</h4>
                                <label class="component-label">"Label Component"</label>
                            </div>
                            <div class="component-item">
                                <h4>"Card"</h4>
                                <div class="component-card">"Card Component"</div>
                            </div>
                            <div class="component-item">
                                <h4>"Separator"</h4>
                                <div class="component-separator"></div>
                            </div>
                        </div>
                    </div>

                    <div class="lazy-components">
                        <h3>"Lazy Loaded Components (On Demand)"</h3>
                        <p class="lazy-description">
                            "These components are not loaded initially. Click 'Load' to dynamically load them:"
                        </p>
                        
                        <div class="lazy-grid">
                            <LazyComponentWrapper name="Alert".to_string() />
                            <LazyComponentWrapper name="Badge".to_string() />
                            <LazyComponentWrapper name="Radio Group".to_string() />
                            <LazyComponentWrapper name="Combobox".to_string() />
                            <LazyComponentWrapper name="Form".to_string() />
                            <LazyComponentWrapper name="Checkbox".to_string() />
                            <LazyComponentWrapper name="Select".to_string() />
                            <LazyComponentWrapper name="Dialog".to_string() />
                            <LazyComponentWrapper name="Tabs".to_string() />
                            <LazyComponentWrapper name="Toast".to_string() />
                            <LazyComponentWrapper name="Tooltip".to_string() />
                            <LazyComponentWrapper name="Popover".to_string() />
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}
