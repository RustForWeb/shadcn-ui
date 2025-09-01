use leptos::prelude::*;
use leptos_style::Style;

const SEPARATOR_CLASS: &str = "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

#[component]
pub fn Separator(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", SEPARATOR_CLASS, class.get().unwrap_or_default())
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
