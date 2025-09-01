use leptos::*;
use leptos::prelude::*;
use crate::lazy_loading::LazyComponentWrapper;
use crate::bundle_analyzer::BundleAnalysisDisplay;
use crate::dynamic_loader::{DynamicLoaderDisplay, DynamicComponentWrapper, DynamicLoader};

#[component]
pub fn App() -> impl IntoView {
    let (current_theme, set_current_theme) = signal("default".to_string());
    let (search_query, set_search_query) = signal("".to_string());
    let (selected_category, set_selected_category) = signal("all".to_string());
    let (show_favorites, set_show_favorites) = signal(false);

    let toggle_theme = move |_| {
        let new_theme = if current_theme.get() == "default" { "new_york" } else { "default" };
        set_current_theme.set(new_theme.to_string());
    };

    let toggle_favorites = move |_| {
        set_show_favorites.update(|f| *f = !*f);
    };
    
    // Create dynamic loader context
    let dynamic_loader = DynamicLoader::new();
    provide_context(dynamic_loader);

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
                        <DynamicLoaderDisplay />
                    </div>
                </div>

                <div class="component-showcase">
                    <div class="showcase-header">
                        <h2>"Component Showcase"</h2>
                        <div class="header-controls">
                            <div class="search-filter-section">
                                <div class="search-bar">
                                    <input 
                                        type="text" 
                                        placeholder="Search components..." 
                                        class="search-input"
                                        on:input={move |ev| {
                                            let value = event_target_value(&ev);
                                            set_search_query.set(value);
                                        }}
                                    />
                                    <div class="search-icon">"🔍"</div>
                                </div>
                                
                                <div class="filter-controls">
                                    <select 
                                        class="category-filter"
                                        on:change={move |ev| {
                                            let value = event_target_value(&ev);
                                            set_selected_category.set(value);
                                        }}
                                    >
                                        <option value="all">"All Categories"</option>
                                        <option value="form">"Form & Input"</option>
                                        <option value="layout">"Layout & Navigation"</option>
                                        <option value="overlay">"Overlay & Feedback"</option>
                                        <option value="data">"Data & Media"</option>
                                    </select>
                                    
                                    <button 
                                        on:click={toggle_favorites} 
                                        class="favorites-btn"
                                        class:active={show_favorites}
                                    >
                                        {if show_favorites.get() { "★" } else { "☆" }}
                                        " Favorites"
                                    </button>
                                </div>
                            </div>
                            
                            <div class="theme-toggle">
                                <button on:click={toggle_theme} class="theme-btn">
                                    {move || format!("Switch to {} Theme", 
                                        if current_theme.get() == "default" { "New York" } else { "Default" }
                                    )}
                                </button>
                            </div>
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
                        
                        <div class="component-categories">
                            <div class="category" data-category="form">
                                <h4>"Form & Input Components"</h4>
                                <div class="lazy-grid">
                                    <LazyComponentWrapper name="Alert".to_string() />
                                    <LazyComponentWrapper name="Badge".to_string() />
                                    <LazyComponentWrapper name="Checkbox".to_string() />
                                    <LazyComponentWrapper name="Combobox".to_string() />
                                    <LazyComponentWrapper name="Form".to_string() />
                                    <LazyComponentWrapper name="Input OTP".to_string() />
                                    <LazyComponentWrapper name="Radio Group".to_string() />
                                    <LazyComponentWrapper name="Select".to_string() />
                                    <LazyComponentWrapper name="Slider".to_string() />
                                    <LazyComponentWrapper name="Switch".to_string() />
                                    <LazyComponentWrapper name="Textarea".to_string() />
                                    <LazyComponentWrapper name="Toggle".to_string() />
                                </div>
                            </div>

                            <div class="category" data-category="layout">
                                <h4>"Layout & Navigation"</h4>
                                <div class="lazy-grid">
                                    <LazyComponentWrapper name="Accordion".to_string() />
                                    <LazyComponentWrapper name="Breadcrumb".to_string() />
                                    <LazyComponentWrapper name="Collapsible".to_string() />
                                    <LazyComponentWrapper name="Command".to_string() />
                                    <LazyComponentWrapper name="Navigation Menu".to_string() />
                                    <LazyComponentWrapper name="Pagination".to_string() />
                                    <LazyComponentWrapper name="Scroll Area".to_string() />
                                    <LazyComponentWrapper name="Separator".to_string() />
                                    <LazyComponentWrapper name="Skeleton".to_string() />
                                    <LazyComponentWrapper name="Tabs".to_string() />
                                </div>
                            </div>

                            <div class="category" data-category="overlay">
                                <h4>"Overlay & Feedback"</h4>
                                <div class="lazy-grid">
                                    <LazyComponentWrapper name="Alert Dialog".to_string() />
                                    <LazyComponentWrapper name="Dialog".to_string() />
                                    <LazyComponentWrapper name="Drawer".to_string() />
                                    <LazyComponentWrapper name="Dropdown Menu".to_string() />
                                    <LazyComponentWrapper name="Hover Card".to_string() />
                                    <LazyComponentWrapper name="Menubar".to_string() />
                                    <LazyComponentWrapper name="Popover".to_string() />
                                    <LazyComponentWrapper name="Sheet".to_string() />
                                    <LazyComponentWrapper name="Toast".to_string() />
                                    <LazyComponentWrapper name="Tooltip".to_string() />
                                </div>
                            </div>

                            <div class="category" data-category="data">
                                <h4>"Data & Media"</h4>
                                <div class="lazy-grid">
                                    <LazyComponentWrapper name="Aspect Ratio".to_string() />
                                    <LazyComponentWrapper name="Calendar".to_string() />
                                    <LazyComponentWrapper name="Carousel".to_string() />
                                    <LazyComponentWrapper name="Context Menu".to_string() />
                                    <LazyComponentWrapper name="Date Picker".to_string() />
                                    <LazyComponentWrapper name="Progress".to_string() />
                                    <LazyComponentWrapper name="Table".to_string() />
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="dynamic-components">
                        <h3>"Dynamic WASM Components (Real Loading)"</h3>
                        <p class="dynamic-description">
                            "These components use real dynamic WASM loading instead of simulation:"
                        </p>
                        
                        <div class="dynamic-grid">
                            <DynamicComponentWrapper name="Button".to_string() />
                            <DynamicComponentWrapper name="Input".to_string() />
                            <DynamicComponentWrapper name="Card".to_string() />
                            <DynamicComponentWrapper name="Modal".to_string() />
                            <DynamicComponentWrapper name="Table".to_string() />
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}
