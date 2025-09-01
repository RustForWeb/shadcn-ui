//! Simple dynamic component loader

use leptos::*;
use leptos::prelude::*;

/// Simple bundle status display
#[component]
pub fn BundleStatusDisplay() -> impl IntoView {
    let (show_bundles, set_show_bundles) = signal(false);
    
    let toggle_bundles = move |_| {
        set_show_bundles.set(!show_bundles.get());
    };

    view! {
        <div class="bundle-status-display">
            <h3>"Bundle Status"</h3>
            
            <div class="bundle-summary">
                <div class="summary-item">
                    <strong>"Initial Bundle:"</strong> "1.08 MB (5 components)"
                </div>
                <div class="summary-item">
                    <strong>"Lazy Available:"</strong> "47 components"
                </div>
                <div class="summary-item">
                    <strong>"Total Available:"</strong> "52 components"
                </div>
            </div>
            
            <button on:click={toggle_bundles} class="bundles-btn">
                {move || if show_bundles.get() { "Hide Bundles" } else { "Show Available Bundles" }}
            </button>
            
            <div class="bundles-section">
                <div class="bundle-list" class:hidden={move || !show_bundles.get()}>
                    <div class="bundle-category">
                        <h4>"Essential Components (Always Loaded)"</h4>
                        <div class="bundle-progress">
                            <div class="progress-bar">
                                <div class="progress-fill essential" style="width: 100%"></div>
                            </div>
                            <span class="progress-text">"5/5 loaded (100%)"</span>
                        </div>
                        <div class="component-list">
                            <div class="component-item loaded">"Button"</div>
                            <div class="component-item loaded">"Input"</div>
                            <div class="component-item loaded">"Label"</div>
                            <div class="component-item loaded">"Card"</div>
                            <div class="component-item loaded">"Separator"</div>
                        </div>
                    </div>
                    
                    <div class="bundle-category">
                        <h4>"Lazy Loaded Components (On Demand)"</h4>
                        <div class="bundle-progress">
                            <div class="progress-bar">
                                <div class="progress-fill lazy" style="width: 0%"></div>
                            </div>
                            <span class="progress-text">"0/47 loaded (0%)"</span>
                        </div>
                        <div class="component-list">
                            <div class="component-item">"Alert"</div>
                            <div class="component-item">"Badge"</div>
                            <div class="component-item">"Radio Group"</div>
                            <div class="component-item">"Combobox"</div>
                            <div class="component-item">"Form"</div>
                            <div class="component-item">"Checkbox"</div>
                            <div class="component-item">"Select"</div>
                            <div class="component-item">"Dialog"</div>
                            <div class="component-item">"Tabs"</div>
                            <div class="component-item">"And 38+ more..."</div>
                        </div>
                    </div>
                    
                    <div class="bundle-info">
                        <h4>"How It Works"</h4>
                        <div class="info-content">
                            <p>"🚀 <strong>Initial Load:</strong> Only essential components are loaded, keeping the initial bundle small."</p>
                            <p>"⚡ <strong>On Demand:</strong> Additional components load when requested by the user."</p>
                            <p>"💾 <strong>Memory Efficient:</strong> Components are only loaded when needed, reducing memory usage."</p>
                            <p>"📱 <strong>Performance:</strong> Faster initial page loads, especially on slow connections."</p>
                        </div>
                    </div>
                </div>
                
                <div class="hidden-placeholder" class:hidden={move || show_bundles.get()}>
                    <span></span>
                </div>
            </div>
        </div>
    }
}
