use leptos::prelude::*;
use leptos_style::Style;

const ALERT_CLASS: &str = "relative w-full rounded-lg border p-4";
const ALERT_TITLE_CLASS: &str = "mb-1 font-medium leading-none tracking-tight";
const ALERT_DESCRIPTION_CLASS: &str = "text-sm [&_p]:leading-relaxed";

/// Alert variant types
#[derive(Debug, Clone, PartialEq)]
pub enum AlertVariant {
    Default,
    Destructive,
    Success,
    Warning,
}

impl Default for AlertVariant {
    fn default() -> Self {
        AlertVariant::Default
    }
}

impl From<AlertVariant> for String {
    fn from(variant: AlertVariant) -> Self {
        match variant {
            AlertVariant::Default => "default".to_string(),
            AlertVariant::Destructive => "destructive".to_string(),
            AlertVariant::Success => "success".to_string(),
            AlertVariant::Warning => "warning".to_string(),
        }
    }
}

#[component]
pub fn Alert(
    #[prop(into, optional)] variant: MaybeProp<AlertVariant>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let variant_class = match variant.get().unwrap_or_default() {
            AlertVariant::Default => "bg-background text-foreground",
            AlertVariant::Destructive => "border-destructive/50 text-destructive dark:border-destructive",
            AlertVariant::Success => "border-green-500/50 text-green-600 dark:text-green-400",
            AlertVariant::Warning => "border-yellow-500/50 text-yellow-600 dark:text-yellow-400",
        };
        
        format!("{} {} {}", ALERT_CLASS, variant_class, class.get().unwrap_or_default())
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

#[component]
pub fn AlertTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", ALERT_TITLE_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <h5
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </h5>
    }
}

#[component]
pub fn AlertDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", ALERT_DESCRIPTION_CLASS, class.get().unwrap_or_default())
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
