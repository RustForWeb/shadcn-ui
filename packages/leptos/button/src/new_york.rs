use leptos::prelude::*;
use leptos_style::Style;

const BUTTON_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

/// Button variant types
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        ButtonVariant::Default
    }
}

impl From<String> for ButtonVariant {
    fn from(s: String) -> Self {
        match s.as_str() {
            "destructive" => ButtonVariant::Destructive,
            "outline" => ButtonVariant::Outline,
            "secondary" => ButtonVariant::Secondary,
            "ghost" => ButtonVariant::Ghost,
            "link" => ButtonVariant::Link,
            _ => ButtonVariant::Default,
        }
    }
}

/// Button size types
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Default
    }
}

impl From<String> for ButtonSize {
    fn from(s: String) -> Self {
        match s.as_str() {
            "sm" => ButtonSize::Sm,
            "lg" => ButtonSize::Lg,
            "icon" => ButtonSize::Icon,
            _ => ButtonSize::Default,
        }
    }
}

/// Props for child components when using as_child
#[derive(Debug, Clone)]
pub struct ButtonChildProps {
    pub class: String,
    pub id: String,
    pub style: String,
    pub disabled: bool,
    pub r#type: String,
    pub onclick: Option<Callback<()>>,
}

#[component]
pub fn Button(
    #[prop(into, optional)] variant: MaybeProp<ButtonVariant>,
    #[prop(into, optional)] size: MaybeProp<ButtonSize>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] as_child: Option<Callback<ButtonChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
    ) -> impl IntoView {
    let handle_click = {
        let on_click = on_click.clone();
        move |_| {
            if let Some(callback) = &on_click {
                callback.run(());
            }
        }
    };

    let computed_class = Signal::derive(move || {
        let variant_class = match variant.get().unwrap_or_default() {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        };
        
        let size_class = match size.get().unwrap_or_default() {
            ButtonSize::Default => "h-10 px-4 py-2",
            ButtonSize::Sm => "h-9 rounded-md px-3",
            ButtonSize::Lg => "h-11 rounded-md px-8",
            ButtonSize::Icon => "h-10 w-10",
        };
        
        format!("{} {} {} {}", BUTTON_CLASS, variant_class, size_class, class.get().unwrap_or_default())
    });

    // Implement as_child functionality using conditional rendering
            if let Some(as_child) = as_child {
            let child_props = ButtonChildProps {
                class: computed_class.get(),
                id: id.get().unwrap_or_default(),
                style: style.get().to_string(),
                disabled: disabled.get(),
                r#type: "button".to_string(),
                onclick: Some(Callback::new(move |_| {
                    if let Some(callback) = &on_click {
                        callback.run(());
                    }
                })),
            };
            as_child.run(child_props).into_any()
        } else {
            view! {
                <button
                    class=computed_class
                    id=id.get().unwrap_or_default()
                    style=move || style.get().to_string()
                    disabled=disabled
                    on:click=handle_click
                >
                    {children.map(|c| c())}
                </button>
            }.into_any()
        }
}
