use leptos::{ev::MouseEvent, prelude::*};
use leptos_style::Style;

const SWITCH_CLASS: &str = "peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input";
const SWITCH_THUMB_CLASS: &str = "pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0";

#[derive(Clone, Copy, PartialEq)]
pub enum SwitchVariant {
    Default,
    Success,
    Warning,
    Destructive,
    Info,
}

impl Default for SwitchVariant {
    fn default() -> Self {
        SwitchVariant::Default
    }
}

impl From<String> for SwitchVariant {
    fn from(s: String) -> Self {
        match s.as_str() {
            "success" => SwitchVariant::Success,
            "warning" => SwitchVariant::Warning,
            "destructive" => SwitchVariant::Destructive,
            "info" => SwitchVariant::Info,
            _ => SwitchVariant::Default,
        }
    }
}

impl SwitchVariant {
    fn checked_class(&self) -> &'static str {
        match self {
            SwitchVariant::Default => "data-[state=checked]:bg-primary",
            SwitchVariant::Success => "data-[state=checked]:bg-green-500",
            SwitchVariant::Warning => "data-[state=checked]:bg-yellow-500",
            SwitchVariant::Destructive => "data-[state=checked]:bg-red-500",
            SwitchVariant::Info => "data-[state=checked]:bg-blue-500",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SwitchSize {
    Sm,
    Md,
    Lg,
}

impl Default for SwitchSize {
    fn default() -> Self {
        SwitchSize::Md
    }
}

impl From<String> for SwitchSize {
    fn from(s: String) -> Self {
        match s.as_str() {
            "sm" => SwitchSize::Sm,
            "lg" => SwitchSize::Lg,
            _ => SwitchSize::Md,
        }
    }
}

impl SwitchSize {
    fn switch_class(&self) -> &'static str {
        match self {
            SwitchSize::Sm => "h-4 w-7",
            SwitchSize::Md => "h-6 w-11",
            SwitchSize::Lg => "h-8 w-14",
        }
    }
    
    fn thumb_class(&self) -> &'static str {
        match self {
            SwitchSize::Sm => "h-3 w-3 data-[state=checked]:translate-x-3",
            SwitchSize::Md => "h-5 w-5 data-[state=checked]:translate-x-5",
            SwitchSize::Lg => "h-6 w-6 data-[state=checked]:translate-x-6",
        }
    }
}

#[component]
pub fn Switch(
    #[prop(into, optional)] checked: Signal<bool>,
    #[prop(into, optional)] on_change: Option<Callback<bool>>,
    #[prop(into, optional)] variant: MaybeProp<SwitchVariant>,
    #[prop(into, optional)] size: MaybeProp<SwitchSize>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] animated: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let switch_variant = variant.get().unwrap_or_default();
    let switch_size = size.get().unwrap_or_default();
    
    let handle_change = {
        let on_change = on_change.clone();
        move |_event: MouseEvent| {
            if let Some(callback) = &on_change {
                let new_value = !checked.get();
                callback.run(new_value);
            }
        }
    };

    let switch_class = switch_size.switch_class();
    let thumb_class = switch_size.thumb_class();
    let variant_class = switch_variant.checked_class();
    let animation_class = if animated.get() { "transition-all duration-200" } else { "transition-colors" };
    
    let computed_class = Signal::derive(move || {
        format!("{} {} {} {} {}", SWITCH_CLASS, switch_class, variant_class, animation_class, class.get().unwrap_or_default())
    });

    let computed_thumb_class = Signal::derive(move || {
        format!("{} {} {}", SWITCH_THUMB_CLASS, thumb_class, animation_class)
    });

    let state_attr = Signal::derive(move || {
        if checked.get() { "checked" } else { "unchecked" }
    });

    view! {
        <button
            type="button"
            role="switch"
            aria-checked={move || checked.get()}
            data-state={state_attr}
            disabled=disabled
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            on:click=handle_change
        >
            <span class=computed_thumb_class data-state={state_attr} />
        </button>
    }
}

// Switch Root with Context
#[derive(Clone, Copy)]
pub struct SwitchContextValue {
    pub checked: RwSignal<bool>,
    pub disabled: RwSignal<bool>,
    pub variant: RwSignal<SwitchVariant>,
    pub size: RwSignal<SwitchSize>,
    pub animated: RwSignal<bool>,
}

#[component]
pub fn SwitchRoot(
    #[prop(into, optional)] checked: Signal<bool>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] variant: MaybeProp<SwitchVariant>,
    #[prop(into, optional)] size: MaybeProp<SwitchSize>,
    #[prop(into, optional)] animated: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let checked_signal = RwSignal::new(checked.get());
    let disabled_signal = RwSignal::new(disabled.get());
    let variant_signal = RwSignal::new(variant.get().unwrap_or_default());
    let size_signal = RwSignal::new(size.get().unwrap_or_default());
    let animated_signal = RwSignal::new(animated.get());

    // Update signals when props change
    Effect::new(move |_| {
        checked_signal.set(checked.get());
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
    Effect::new(move |_| {
        animated_signal.set(animated.get());
    });

    let context_value = SwitchContextValue {
        checked: checked_signal,
        disabled: disabled_signal,
        variant: variant_signal,
        size: size_signal,
        animated: animated_signal,
    };

    provide_context(context_value);

    view! {
        <div class="flex items-center space-x-2">
            {children.map(|c| c())}
        </div>
    }
}

// Switch Thumb (uses context)
#[component]
pub fn SwitchThumb(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let ctx = expect_context::<SwitchContextValue>();
    
    let thumb_class = ctx.size.get().thumb_class();
    let animation_class = if ctx.animated.get() { "transition-all duration-200" } else { "transition-transform" };
    let state_attr = Signal::derive(move || {
        if ctx.checked.get() { "checked" } else { "unchecked" }
    });
    
    let computed_class = Signal::derive(move || {
        format!("{} {} {} {}", SWITCH_THUMB_CLASS, thumb_class, animation_class, class.get().unwrap_or_default())
    });

    view! {
        <span
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            data-state={state_attr}
        />
    }
}

// Switch Label
#[component]
pub fn SwitchLabel(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 {}", class.get().unwrap_or_default())
    });

    view! {
        <label
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </label>
    }
}
