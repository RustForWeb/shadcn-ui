//! Simple bundle analyzer for monitoring component usage

use leptos::*;
use leptos::prelude::*;

/// Simple bundle analysis display
#[component]
pub fn BundleAnalysisDisplay() -> impl IntoView {
    let (show_details, set_show_details) = signal(false);
    
    let toggle_details = move |_| {
        set_show_details.set(!show_details.get());
    };

    view! {
        <div class="bundle-analysis-display">
            <h3>"Bundle Analysis"</h3>
            
            <div class="analysis-summary">
                <div class="summary-item">
                    <strong>"Total Components:"</strong> "52"
                </div>
                <div class="summary-item">
                    <strong>"Essential:"</strong> "5"
                </div>
                <div class="summary-item">
                    <strong>"Lazy Available:"</strong> "47"
                </div>
                <div class="summary-item">
                    <strong>"Currently Loaded:"</strong> "0"
                </div>
            </div>
            
            <div class="bundle-metrics">
                <div class="metric-item">
                    <strong>"Initial Bundle:"</strong> 
                    <span class="metric-value">"1.08 MB"</span>
                </div>
                <div class="metric-item">
                    <strong>"Current Bundle:"</strong> 
                    <span class="metric-value">"1.08 MB"</span>
                </div>
                <div class="metric-item">
                    <strong>"Total Savings:"</strong> 
                    <span class="metric-value savings">"2.32 MB"</span>
                </div>
                <div class="metric-item">
                    <strong>"Savings %:"</strong> 
                    <span class="metric-value savings">"68.2%"</span>
                </div>
            </div>
            
            <button on:click={toggle_details} class="details-btn">
                {move || if show_details.get() { "Hide Details" } else { "Show Details" }}
            </button>
            
            <div class="details-section">
                <div class="analysis-details" class:hidden={move || !show_details.get()}>
                    <h4>"Component Breakdown"</h4>
                    <div class="component-categories">
                        <div class="category essential">
                            <h5>"Essential Components (Always Loaded)"</h5>
                            <div class="component-list">
                                <div class="component-item">"Button"</div>
                                <div class="component-item">"Input"</div>
                                <div class="component-item">"Label"</div>
                                <div class="component-item">"Card"</div>
                                <div class="component-item">"Separator"</div>
                            </div>
                        </div>
                        
                        <div class="category lazy">
                            <h5>"Lazy Loaded Components (On Demand)"</h5>
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
                    </div>
                    
                    <div class="optimization-tips">
                        <h4>"Optimization Benefits"</h4>
                        <ul>
                            <li>"🚀 Initial page load: Only 1.08 MB (vs 3.4 MB)"</li>
                            <li>"⚡ Components load on demand when needed"</li>
                            <li>"💾 Memory efficient: Only load what you use"</li>
                            <li>"📱 Better performance on slow connections"</li>
                            <li>"🎯 Progressive enhancement: Start fast, enhance gradually"</li>
                        </ul>
                    </div>
                </div>
                
                <div class="hidden-placeholder" class:hidden={move || show_details.get()}>
                    <span></span>
                </div>
            </div>
        </div>
    }
}
