//! Simple lazy loading component

use leptos::*;
use leptos::prelude::*;

/// Simple lazy component wrapper
#[component]
pub fn LazyComponentWrapper(
    #[prop(into)] name: String,
) -> impl IntoView {
    let (is_loaded, set_is_loaded) = signal(false);
    
    let load_component = move |_| {
        set_is_loaded.set(true);
    };

    view! {
        <div class="lazy-component-wrapper">
            <h4>{name.clone()}</h4>
            
            <div class="component-content">
                <div class="lazy-component-loaded" class:hidden={move || !is_loaded.get()}>
                    <p>"✅ Component loaded successfully!"</p>
                    <div class="component-demo">
                        <span>"🎉 {name} is now available"</span>
                    </div>
                </div>
                
                <div class="component-placeholder" class:hidden={move || is_loaded.get()}>
                    <p>"This component is not yet loaded. Click to load it on demand."</p>
                    <button on:click={load_component} class="load-btn">
                        "Load {name}"
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Simple lazy loading provider
#[component]
pub fn LazyLoadingProvider(
    #[prop(into)] children: Children,
) -> impl IntoView {
    view! {
        <div class="lazy-loading-provider">
            {children()}
        </div>
    }
}
