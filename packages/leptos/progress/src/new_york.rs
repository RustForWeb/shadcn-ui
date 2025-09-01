use leptos::prelude::*;
use leptos_style::Style;

const PROGRESS_CLASS: &str = "relative w-full overflow-hidden rounded-full bg-secondary";
const PROGRESS_INDICATOR_CLASS: &str = "h-full w-full flex-1 bg-primary transition-all";

#[derive(Clone, Copy, PartialEq)]
pub enum ProgressVariant {
    Default,
    Success,
    Warning,
    Destructive,
    Info,
}

impl Default for ProgressVariant {
    fn default() -> Self {
        ProgressVariant::Default
    }
}

impl From<String> for ProgressVariant {
    fn from(s: String) -> Self {
        match s.as_str() {
            "success" => ProgressVariant::Success,
            "warning" => ProgressVariant::Warning,
            "destructive" => ProgressVariant::Destructive,
            "info" => ProgressVariant::Info,
            _ => ProgressVariant::Default,
        }
    }
}

impl ProgressVariant {
    fn indicator_class(&self) -> &'static str {
        match self {
            ProgressVariant::Default => "bg-primary",
            ProgressVariant::Success => "bg-green-500",
            ProgressVariant::Warning => "bg-yellow-500",
            ProgressVariant::Destructive => "bg-red-500",
            ProgressVariant::Info => "bg-blue-500",
        }
    }
}

#[component]
pub fn Progress(
    #[prop(into, optional)] value: Signal<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] variant: MaybeProp<ProgressVariant>,
    #[prop(into, optional)] animated: Signal<bool>,
    #[prop(into, optional)] show_label: Signal<bool>,
    #[prop(into, optional)] size: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let max_value = max.get().unwrap_or(100.0);
    let progress_variant = variant.get().unwrap_or_default();
    let size_class = match size.get().unwrap_or_default().as_str() {
        "sm" => "h-2",
        "lg" => "h-4",
        "xl" => "h-6",
        _ => "h-3",
    };
    
    let progress_percentage = Signal::derive(move || {
        let val = value.get();
        let max_val = max_value;
        if max_val <= 0.0 { 0.0 } else { (val / max_val * 100.0).clamp(0.0, 100.0) }
    });

    let indicator_class = Signal::derive(move || {
        let base_class = PROGRESS_INDICATOR_CLASS;
        let variant_class = progress_variant.indicator_class();
        let animation_class = if animated.get() { "animate-pulse" } else { "" };
        format!("{} {} {}", base_class, variant_class, animation_class)
    });

    let computed_class = Signal::derive(move || {
        format!("{} {} {}", PROGRESS_CLASS, size_class, class.get().unwrap_or_default())
    });

    view! {
        <div class="w-full space-y-2">
            <div
                class=computed_class
                id=id.get().unwrap_or_default()
                style=move || style.get().to_string()
                role="progressbar"
                aria-valuenow={move || value.get()}
                aria-valuemin="0"
                aria-valuemax={max_value}
            >
                <div
                    class=indicator_class
                    style={move || format!("width: {}%", progress_percentage.get())}
                />
            </div>
            <Show
                when=move || show_label.get()
                fallback=|| view! { <div class="hidden"></div> }
            >
                <div class="flex justify-between text-sm text-muted-foreground">
                    <span>"Progress"</span>
                    <span>{move || format!("{:.0}%", progress_percentage.get())}</span>
                </div>
            </Show>
        </div>
    }
}

// Progress Root with Context
#[derive(Clone, Copy)]
pub struct ProgressContextValue {
    pub value: RwSignal<f64>,
    pub max: RwSignal<f64>,
    pub variant: RwSignal<ProgressVariant>,
    pub animated: RwSignal<bool>,
    pub show_label: RwSignal<bool>,
}

#[component]
pub fn ProgressRoot(
    #[prop(into, optional)] value: Signal<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] variant: MaybeProp<ProgressVariant>,
    #[prop(into, optional)] animated: Signal<bool>,
    #[prop(into, optional)] show_label: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let value_signal = RwSignal::new(value.get());
    let max_signal = RwSignal::new(max.get().unwrap_or(100.0));
    let variant_signal = RwSignal::new(variant.get().unwrap_or_default());
    let animated_signal = RwSignal::new(animated.get());
    let show_label_signal = RwSignal::new(show_label.get());

    // Update signals when props change
    Effect::new(move |_| {
        value_signal.set(value.get());
    });
    Effect::new(move |_| {
        max_signal.set(max.get().unwrap_or(100.0));
    });
    Effect::new(move |_| {
        variant_signal.set(variant.get().unwrap_or_default());
    });
    Effect::new(move |_| {
        animated_signal.set(animated.get());
    });
    Effect::new(move |_| {
        show_label_signal.set(show_label.get());
    });

    let context_value = ProgressContextValue {
        value: value_signal,
        max: max_signal,
        variant: variant_signal,
        animated: animated_signal,
        show_label: show_label_signal,
    };

    provide_context(context_value);

    view! {
        <div class="w-full">
            {children.map(|c| c())}
        </div>
    }
}

// Progress Indicator (uses context)
#[component]
pub fn ProgressIndicator(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let ctx = expect_context::<ProgressContextValue>();
    
    let progress_percentage = Signal::derive(move || {
        let val = ctx.value.get();
        let max_val = ctx.max.get();
        if max_val <= 0.0 { 0.0 } else { (val / max_val * 100.0).clamp(0.0, 100.0) }
    });

    let indicator_class = Signal::derive(move || {
        let base_class = PROGRESS_INDICATOR_CLASS;
        let variant_class = ctx.variant.get().indicator_class();
        let animation_class = if ctx.animated.get() { "animate-pulse" } else { "" };
        format!("{} {} {} {}", base_class, variant_class, animation_class, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=indicator_class
            id=id.get().unwrap_or_default()
            style={move || format!("width: {}%; {}", progress_percentage.get(), style.get().to_string())}
        />
    }
}

// Progress Label (uses context)
#[component]
pub fn ProgressLabel(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let ctx = expect_context::<ProgressContextValue>();
    
    let progress_percentage = Signal::derive(move || {
        let val = ctx.value.get();
        let max_val = ctx.max.get();
        if max_val <= 0.0 { 0.0 } else { (val / max_val * 100.0).clamp(0.0, 100.0) }
    });

    let computed_class = Signal::derive(move || {
        format!("flex justify-between text-sm text-muted-foreground {}", class.get().unwrap_or_default())
    });

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            <span>"Progress"</span>
            <span>{move || format!("{:.0}%", progress_percentage.get())}</span>
        </div>
    }
}
