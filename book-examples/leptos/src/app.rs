use leptos::*;
use leptos::prelude::*;
use crate::lazy_loading::LazyComponentWrapper;

#[component]
pub fn App() -> impl IntoView {
    let (current_theme, set_current_theme) = signal("default".to_string());
    let (search_query, set_search_query) = signal("".to_string());
    let (selected_category, set_selected_category) = signal("all".to_string());

    let toggle_theme = move |_| {
        let new_theme = if current_theme.get_untracked() == "default" { "new_york" } else { "default" };
        set_current_theme.set(new_theme.to_string());
    };

    view! {
        <div class="app" data-theme={current_theme}>
            <header class="app-header">
                <h1>"ShadCN UI - Enhanced Lazy Loading Demo"</h1>
                <button on:click={toggle_theme} class="theme-toggle">
                    "Toggle Theme"
                </button>
            </header>

            <main class="app-main">
                <div class="container">
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
                        </div>
                    </div>

                    <div class="essential-components">
                        <h3>"Essential Components (Always Loaded)"</h3>
                        <div class="component-grid">
                            <div class="component-item">
                                <h4>"Button"</h4>
                                <button class="btn btn-primary">"Button Component"</button>
                            </div>
                            <div class="component-item">
                                <h4>"Input"</h4>
                                <input class="input" placeholder="Input Component" />
                            </div>
                            <div class="component-item">
                                <h4>"Label"</h4>
                                <label class="label">"Label Component"</label>
                            </div>
                        </div>
                    </div>

                    <div class="lazy-test-section">
                        <h3>"Lazy Loading Test (With Search & Filter)"</h3>
                        <p>"Testing if search/filter interactions cause memory issues:"</p>
                        <div class="lazy-grid">
                            <LazyComponentWrapper name="Alert" />
                            <LazyComponentWrapper name="Badge" />
                            <LazyComponentWrapper name="Card" />
                            <LazyComponentWrapper name="Dialog" />
                            <LazyComponentWrapper name="Form" />
                        </div>
                    </div>

                    <div class="simple-section">
                        <h3>"Simple Test Section"</h3>
                        <p>"This is a simplified version to isolate memory issues."</p>
                        <div class="test-content">
                            <p>"If you can see this, basic rendering is working."</p>
                            <button class="btn btn-secondary">"Test Button"</button>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}
