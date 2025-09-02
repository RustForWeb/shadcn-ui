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
        let new_theme = if current_theme.get_untracked() == "default" { "new_york" } else { "default" };
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
                                            let value = ev.target().unwrap().value_of().as_string().unwrap_or_default();
                                            set_search_query.set(value);
                                        }}
                                    />
                                    <div class="search-icon">"🔍"</div>
                                </div>
                                
                                <div class="filter-controls">
                                    <select 
                                        class="category-filter"
                                        on:change={move |ev| {
                                            let value = ev.target().unwrap().value_of().as_string().unwrap_or_default();
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
                                        class:active={move || show_favorites.get()}
                                    >
                                        {move || if show_favorites.get() { "★" } else { "☆" }}
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
                                    <LazyComponentWrapper name="Alert" />
                                    <LazyComponentWrapper name="Badge" />
                                    <LazyComponentWrapper name="Checkbox" />
                                    <LazyComponentWrapper name="Combobox" />
                                    <LazyComponentWrapper name="Form" />
                                    <LazyComponentWrapper name="Input OTP" />
                                    <LazyComponentWrapper name="Radio Group" />
                                    <LazyComponentWrapper name="Select" />
                                    <LazyComponentWrapper name="Slider" />
                                    <LazyComponentWrapper name="Switch" />
                                    <LazyComponentWrapper name="Textarea" />
                                    <LazyComponentWrapper name="Toggle" />
                                </div>
                            </div>

                            <div class="category" data-category="layout">
                                <h4>"Layout & Navigation"</h4>
                                <div class="lazy-grid">
                                    <LazyComponentWrapper name="Accordion" />
                                    <LazyComponentWrapper name="Breadcrumb" />
                                    <LazyComponentWrapper name="Collapsible" />
                                    <LazyComponentWrapper name="Command" />
                                    <LazyComponentWrapper name="Navigation Menu" />
                                    <LazyComponentWrapper name="Pagination" />
                                    <LazyComponentWrapper name="Scroll Area" />
                                    <LazyComponentWrapper name="Separator" />
                                    <LazyComponentWrapper name="Skeleton" />
                                    <LazyComponentWrapper name="Tabs" />
                                </div>
                            </div>

                            <div class="category" data-category="overlay">
                                <h4>"Overlay & Feedback"</h4>
                                <div class="lazy-grid">
                                    <LazyComponentWrapper name="Alert Dialog" />
                                    <LazyComponentWrapper name="Dialog" />
                                    <LazyComponentWrapper name="Drawer" />
                                    <LazyComponentWrapper name="Dropdown Menu" />
                                    <LazyComponentWrapper name="Hover Card" />
                                    <LazyComponentWrapper name="Menubar" />
                                    <LazyComponentWrapper name="Popover" />
                                    <LazyComponentWrapper name="Sheet" />
                                    <LazyComponentWrapper name="Toast" />
                                    <LazyComponentWrapper name="Tooltip" />
                                </div>
                            </div>

                            <div class="category" data-category="data">
                                <h4>"Data & Media"</h4>
                                <div class="lazy-grid">
                                    <LazyComponentWrapper name="Aspect Ratio" />
                                    <LazyComponentWrapper name="Calendar" />
                                    <LazyComponentWrapper name="Carousel" />
                                    <LazyComponentWrapper name="Context Menu" />
                                    <LazyComponentWrapper name="Date Picker" />
                                    <LazyComponentWrapper name="Progress" />
                                    <LazyComponentWrapper name="Table" />
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
