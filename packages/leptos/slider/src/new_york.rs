use leptos::{ev::Event, prelude::*};
use leptos_style::Style;
use leptos::wasm_bindgen::JsCast;

const SLIDER_CLASS: &str = "relative flex w-full touch-none select-none items-center";
const SLIDER_TRACK_CLASS: &str = "relative h-2 w-full grow overflow-hidden rounded-full bg-secondary";
const SLIDER_RANGE_CLASS: &str = "absolute h-full bg-primary";
const SLIDER_THUMB_CLASS: &str = "block h-5 w-5 rounded-full border-2 border-primary bg-background ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

#[derive(Clone, Copy, PartialEq)]
pub enum SliderVariant {
    Default,
    Success,
    Warning,
    Destructive,
    Info,
}

impl Default for SliderVariant {
    fn default() -> Self {
        SliderVariant::Default
    }
}

impl From<String> for SliderVariant {
    fn from(s: String) -> Self {
        match s.as_str() {
            "success" => SliderVariant::Success,
            "warning" => SliderVariant::Warning,
            "destructive" => SliderVariant::Destructive,
            "info" => SliderVariant::Info,
            _ => SliderVariant::Default,
        }
    }
}

impl SliderVariant {
    fn range_class(&self) -> &'static str {
        match self {
            SliderVariant::Default => "bg-primary",
            SliderVariant::Success => "bg-green-500",
            SliderVariant::Warning => "bg-yellow-500",
            SliderVariant::Destructive => "bg-red-500",
            SliderVariant::Info => "bg-blue-500",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SliderSize {
    Sm,
    Md,
    Lg,
}

impl Default for SliderSize {
    fn default() -> Self {
        SliderSize::Md
    }
}

impl From<String> for SliderSize {
    fn from(s: String) -> Self {
        match s.as_str() {
            "sm" => SliderSize::Sm,
            "lg" => SliderSize::Lg,
            _ => SliderSize::Md,
        }
    }
}

impl SliderSize {
    fn track_class(&self) -> &'static str {
        match self {
            SliderSize::Sm => "h-1",
            SliderSize::Md => "h-2",
            SliderSize::Lg => "h-3",
        }
    }
    
    fn thumb_class(&self) -> &'static str {
        match self {
            SliderSize::Sm => "h-3 w-3",
            SliderSize::Md => "h-5 w-5",
            SliderSize::Lg => "h-6 w-6",
        }
    }
}

#[component]
pub fn Slider(
    #[prop(into, optional)] value: Signal<f64>,
    #[prop(into, optional)] min: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] step: MaybeProp<f64>,
    #[prop(into, optional)] on_change: Option<Callback<f64>>,
    #[prop(into, optional)] variant: MaybeProp<SliderVariant>,
    #[prop(into, optional)] size: MaybeProp<SliderSize>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] show_value: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let min_value = min.get().unwrap_or(0.0);
    let max_value = max.get().unwrap_or(100.0);
    let step_value = step.get().unwrap_or(1.0);
    let slider_variant = variant.get().unwrap_or_default();
    let slider_size = size.get().unwrap_or_default();
    
    let handle_change = {
        let on_change = on_change.clone();
        move |event: Event| {
            if let Some(callback) = &on_change {
                let target = event.target().unwrap();
                let input = target.unchecked_into::<web_sys::HtmlInputElement>();
                if let Ok(val) = input.value().parse::<f64>() {
                    callback.run(val);
                }
            }
        }
    };

    let progress_percentage = Signal::derive(move || {
        let val = value.get();
        let range = max_value - min_value;
        if range <= 0.0 { 0.0 } else { ((val - min_value) / range * 100.0).clamp(0.0, 100.0) }
    });

    let track_class = slider_size.track_class();
    let thumb_class = slider_size.thumb_class();
    let variant_class = slider_variant.range_class();
    
    let computed_class = Signal::derive(move || {
        format!("{} {} {}", SLIDER_CLASS, track_class, class.get().unwrap_or_default())
    });

    let computed_range_class = Signal::derive(move || {
        format!("{} {} {}", SLIDER_RANGE_CLASS, variant_class, track_class)
    });

    let computed_thumb_class = Signal::derive(move || {
        format!("{} {} {}", SLIDER_THUMB_CLASS, thumb_class, track_class)
    });

    view! {
        <div class="w-full space-y-2">
            <div
                class=computed_class
                id=id.get().unwrap_or_default()
                style=move || style.get().to_string()
            >
                <div class={format!("{} {}", SLIDER_TRACK_CLASS, track_class)}>
                    <div
                        class=computed_range_class
                        style={move || format!("width: {}%", progress_percentage.get())}
                    />
                </div>
                <input
                    type="range"
                    min={min_value}
                    max={max_value}
                    step={step_value}
                    value={move || value.get()}
                    disabled=disabled
                    class="absolute inset-0 h-full w-full opacity-0 cursor-pointer"
                    on:input=handle_change
                />
                <div
                    class=computed_thumb_class
                    style={move || format!("left: {}%", progress_percentage.get())}
                />
            </div>
            <Show
                when=move || show_value.get()
                fallback=|| view! { <div class="hidden"></div> }
            >
                <div class="flex justify-between text-sm text-muted-foreground">
                    <span>{move || format!("{:.0}", min_value)}</span>
                    <span>{move || format!("{:.0}", value.get())}</span>
                    <span>{move || format!("{:.0}", max_value)}</span>
                </div>
            </Show>
        </div>
    }
}

// Range Slider (for dual values)
#[component]
pub fn RangeSlider(
    #[prop(into, optional)] values: Signal<(f64, f64)>,
    #[prop(into, optional)] min: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] step: MaybeProp<f64>,
    #[prop(into, optional)] _on_change: Option<Callback<(f64, f64)>>,
    #[prop(into, optional)] variant: MaybeProp<SliderVariant>,
    #[prop(into, optional)] size: MaybeProp<SliderSize>,
    #[prop(into, optional)] _disabled: Signal<bool>,
    #[prop(into, optional)] show_values: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let min_value = min.get().unwrap_or(0.0);
    let max_value = max.get().unwrap_or(100.0);
    let _step_value = step.get().unwrap_or(1.0);
    let slider_variant = variant.get().unwrap_or_default();
    let slider_size = size.get().unwrap_or_default();
    
    let (_min_val, _max_val) = values.get();
    let range_percentage = Signal::derive(move || {
        let (min_v, max_v) = values.get();
        let range = max_value - min_value;
        if range <= 0.0 { (0.0, 0.0) } else {
            let min_percent = ((min_v - min_value) / range * 100.0).clamp(0.0, 100.0);
            let max_percent = ((max_v - min_value) / range * 100.0).clamp(0.0, 100.0);
            (min_percent, max_percent)
        }
    });

    let track_class = slider_size.track_class();
    let thumb_class = slider_size.thumb_class();
    let variant_class = slider_variant.range_class();
    
    let computed_class = Signal::derive(move || {
        format!("{} {} {}", SLIDER_CLASS, track_class, class.get().unwrap_or_default())
    });

    let computed_range_class = Signal::derive(move || {
        format!("{} {} {}", SLIDER_RANGE_CLASS, variant_class, track_class)
    });

    let computed_thumb_class = Signal::derive(move || {
        format!("{} {} {}", SLIDER_THUMB_CLASS, thumb_class, track_class)
    });

    view! {
        <div class="w-full space-y-2">
            <div
                class=computed_class
                id=id.get().unwrap_or_default()
                style=move || style.get().to_string()
            >
                <div class={format!("{} {}", SLIDER_TRACK_CLASS, track_class)}>
                    <div
                        class=computed_range_class
                        style={move || {
                            let (min_p, max_p) = range_percentage.get();
                            format!("left: {}%; width: {}%", min_p, max_p - min_p)
                        }}
                    />
                </div>
                <div
                    class=computed_thumb_class
                    style={move || format!("left: {}%", range_percentage.get().0)}
                />
                <div
                    class=computed_thumb_class
                    style={move || format!("left: {}%", range_percentage.get().1)}
                />
            </div>
            <Show
                when=move || show_values.get()
                fallback=|| view! { <div class="hidden"></div> }
            >
                <div class="flex justify-between text-sm text-muted-foreground">
                    <span>{move || format!("{:.0}", min_value)}</span>
                    <span>{move || format!("{:.0} - {:.0}", values.get().0, values.get().1)}</span>
                    <span>{move || format!("{:.0}", max_value)}</span>
                </div>
            </Show>
        </div>
    }
}

// Slider Root with Context
#[derive(Clone, Copy)]
pub struct SliderContextValue {
    pub value: RwSignal<f64>,
    pub min: RwSignal<f64>,
    pub max: RwSignal<f64>,
    pub step: RwSignal<f64>,
    pub disabled: RwSignal<bool>,
    pub variant: RwSignal<SliderVariant>,
    pub size: RwSignal<SliderSize>,
}

#[component]
pub fn SliderRoot(
    #[prop(into, optional)] value: Signal<f64>,
    #[prop(into, optional)] min: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] step: MaybeProp<f64>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] variant: MaybeProp<SliderVariant>,
    #[prop(into, optional)] size: MaybeProp<SliderSize>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let value_signal = RwSignal::new(value.get());
    let min_signal = RwSignal::new(min.get().unwrap_or(0.0));
    let max_signal = RwSignal::new(max.get().unwrap_or(100.0));
    let step_signal = RwSignal::new(step.get().unwrap_or(1.0));
    let disabled_signal = RwSignal::new(disabled.get());
    let variant_signal = RwSignal::new(variant.get().unwrap_or_default());
    let size_signal = RwSignal::new(size.get().unwrap_or_default());

    // Update signals when props change
    Effect::new(move |_| {
        value_signal.set(value.get());
    });
    Effect::new(move |_| {
        min_signal.set(min.get().unwrap_or(0.0));
    });
    Effect::new(move |_| {
        max_signal.set(max.get().unwrap_or(100.0));
    });
    Effect::new(move |_| {
        step_signal.set(step.get().unwrap_or(1.0));
    });
    Effect::new(move |_| {
        disabled_signal.set(disabled.get());
    });
    Effect::new(move |_| {
        variant_signal.set(variant.get().unwrap_or_default());
    });
    Effect::new(move |_| {
        size_signal.set(size.get().unwrap_or_default());
    });

    let context_value = SliderContextValue {
        value: value_signal,
        min: min_signal,
        max: max_signal,
        step: step_signal,
        disabled: disabled_signal,
        variant: variant_signal,
        size: size_signal,
    };

    provide_context(context_value);

    view! {
        <div class="w-full">
            {children.map(|c| c())}
        </div>
    }
}
