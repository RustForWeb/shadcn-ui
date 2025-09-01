use leptos::prelude::*;
use leptos_style::Style;

const TOAST_CLASS: &str = "relative w-full rounded-lg border p-4";

#[component]
pub fn Toast(
    #[prop(into, optional)] variant: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let variant_class = match variant.get().unwrap_or_default().as_str() {
            "default" => "bg-background text-foreground",
            "destructive" => "border-destructive/50 text-destructive dark:border-destructive",
            "success" => "border-green-500/50 text-green-600 dark:text-green-400",
            "warning" => "border-yellow-500/50 text-yellow-600 dark:text-yellow-400",
            _ => "bg-background text-foreground",
        };
        
        format!("{} {} {}", TOAST_CLASS, variant_class, class.get().unwrap_or_default())
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
