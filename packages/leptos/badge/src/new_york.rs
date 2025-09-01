use leptos::prelude::*;
use leptos_style::Style;

const BADGE_CLASS: &str = "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2";

/// Badge variant types
#[derive(Debug, Clone, PartialEq)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
}

impl Default for BadgeVariant {
    fn default() -> Self {
        BadgeVariant::Default
    }
}

impl From<BadgeVariant> for String {
    fn from(variant: BadgeVariant) -> Self {
        match variant {
            BadgeVariant::Default => "default".to_string(),
            BadgeVariant::Secondary => "secondary".to_string(),
            BadgeVariant::Destructive => "destructive".to_string(),
            BadgeVariant::Outline => "outline".to_string(),
        }
    }
}

#[component]
pub fn Badge(
    #[prop(into, optional)] variant: MaybeProp<BadgeVariant>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let variant_class = match variant.get().unwrap_or_default() {
            BadgeVariant::Default => "border-transparent bg-primary text-primary-foreground hover:bg-primary/80",
            BadgeVariant::Secondary => "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
            BadgeVariant::Destructive => "border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/80",
            BadgeVariant::Outline => "text-foreground",
        };
        
        format!("{} {} {}", BADGE_CLASS, variant_class, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}
