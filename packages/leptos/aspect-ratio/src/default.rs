use leptos::prelude::*;
use leptos_style::Style;

#[component]
pub fn AspectRatio(
    /// The desired ratio (e.g., 16/9, 4/3, 1)
    #[prop(default = 1.0)]
    ratio: f64,
    
    // Global attributes
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    id: Option<String>,
    #[prop(default = Style::new())]
    style: Style,
    
    children: Children,
) -> impl IntoView {
    // Calculate padding-bottom as percentage for aspect ratio
    let padding_bottom = (1.0 / ratio) * 100.0;
    
    let computed_style = format!(
        "position: relative; width: 100%; padding-bottom: {}%; {}",
        padding_bottom,
        style.to_string()
    );
    
    view! {
        <div
            class=class.clone().unwrap_or_default()
            id=id.clone()
            style=computed_style
        >
            <div 
                class="absolute inset-0"
                style="position: absolute; top: 0; right: 0; bottom: 0; left: 0;"
            >
                {children()}
            </div>
        </div>
    }
}