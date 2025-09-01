use leptos::prelude::*;
use leptos_style::Style;

const SKELETON_CLASS: &str = "animate-pulse rounded-md bg-muted";

#[derive(Clone, Copy, PartialEq)]
pub enum SkeletonVariant {
    Default,
    Text,
    Avatar,
    Button,
    Card,
    Image,
}

impl Default for SkeletonVariant {
    fn default() -> Self {
        SkeletonVariant::Default
    }
}

impl From<String> for SkeletonVariant {
    fn from(s: String) -> Self {
        match s.as_str() {
            "text" => SkeletonVariant::Text,
            "avatar" => SkeletonVariant::Avatar,
            "button" => SkeletonVariant::Button,
            "card" => SkeletonVariant::Card,
            "image" => SkeletonVariant::Image,
            _ => SkeletonVariant::Default,
        }
    }
}

impl SkeletonVariant {
    fn base_class(&self) -> &'static str {
        match self {
            SkeletonVariant::Default => "h-4 w-full",
            SkeletonVariant::Text => "h-4 w-full",
            SkeletonVariant::Avatar => "h-12 w-12 rounded-full",
            SkeletonVariant::Button => "h-10 w-20",
            SkeletonVariant::Card => "h-32 w-full",
            SkeletonVariant::Image => "h-48 w-full",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SkeletonSize {
    Sm,
    Md,
    Lg,
    Xl,
}

impl Default for SkeletonSize {
    fn default() -> Self {
        SkeletonSize::Md
    }
}

impl From<String> for SkeletonSize {
    fn from(s: String) -> Self {
        match s.as_str() {
            "sm" => SkeletonSize::Sm,
            "lg" => SkeletonSize::Lg,
            "xl" => SkeletonSize::Xl,
            _ => SkeletonSize::Md,
        }
    }
}

impl SkeletonSize {
    fn height_class(&self) -> &'static str {
        match self {
            SkeletonSize::Sm => "h-2",
            SkeletonSize::Md => "h-4",
            SkeletonSize::Lg => "h-6",
            SkeletonSize::Xl => "h-8",
        }
    }
}

#[component]
pub fn Skeleton(
    #[prop(into, optional)] variant: MaybeProp<SkeletonVariant>,
    #[prop(into, optional)] size: MaybeProp<SkeletonSize>,
    #[prop(into, optional)] animated: Signal<bool>,
    #[prop(into, optional)] width: MaybeProp<String>,
    #[prop(into, optional)] height: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let skeleton_variant = variant.get().unwrap_or_default();
    let skeleton_size = size.get().unwrap_or_default();
    
    let base_class = skeleton_variant.base_class();
    let size_class = skeleton_size.height_class();
    let animation_class = if animated.get() { "animate-pulse" } else { "" };
    
    let computed_class = Signal::derive(move || {
        let variant_class = if skeleton_variant == SkeletonVariant::Default {
            size_class
        } else {
            base_class
        };
        format!("{} {} {} {}", SKELETON_CLASS, variant_class, animation_class, class.get().unwrap_or_default())
    });

    let computed_style = Signal::derive(move || {
        let mut style_str = style.get().to_string();
        if let Some(w) = width.get() {
            if !w.is_empty() {
                style_str = format!("{} width: {};", style_str, w);
            }
        }
        if let Some(h) = height.get() {
            if !h.is_empty() {
                style_str = format!("{} height: {};", style_str, h);
            }
        }
        style_str
    });

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=computed_style
        />
    }
}

// Skeleton Text (specialized for text content)
#[component]
pub fn SkeletonText(
    #[prop(into, optional)] lines: MaybeProp<usize>,
    #[prop(into, optional)] size: MaybeProp<SkeletonSize>,
    #[prop(into, optional)] animated: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let line_count = lines.get().unwrap_or(3);
    let skeleton_size = size.get().unwrap_or_default();
    let size_class = skeleton_size.height_class();
    let animation_class = if animated.get() { "animate-pulse" } else { "" };
    
    let computed_class = Signal::derive(move || {
        format!("{} {} {} {}", SKELETON_CLASS, size_class, animation_class, class.get().unwrap_or_default())
    });

    view! {
        <div class="space-y-2" id=id.get().unwrap_or_default() style=move || style.get().to_string()>
            {move || (0..line_count).map(|i| {
                let width_class = if i == line_count - 1 { "w-3/4" } else { "w-full" };
                view! {
                    <div class={format!("{} {}", computed_class.get(), width_class)} />
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

// Skeleton Avatar (specialized for avatar/icon placeholders)
#[component]
pub fn SkeletonAvatar(
    #[prop(into, optional)] size: MaybeProp<String>,
    #[prop(into, optional)] animated: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let size_class = match size.get().unwrap_or_default().as_str() {
        "sm" => "h-8 w-8",
        "lg" => "h-16 w-16",
        "xl" => "h-20 w-20",
        _ => "h-12 w-12",
    };
    let animation_class = if animated.get() { "animate-pulse" } else { "" };
    
    let computed_class = Signal::derive(move || {
        format!("{} {} rounded-full {} {}", SKELETON_CLASS, size_class, animation_class, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        />
    }
}

// Skeleton Card (specialized for card placeholders)
#[component]
pub fn SkeletonCard(
    #[prop(into, optional)] animated: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let animation_class = if animated.get() { "animate-pulse" } else { "" };
    
    let computed_class = Signal::derive(move || {
        format!("{} h-32 w-full rounded-lg {} {}", SKELETON_CLASS, animation_class, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        />
    }
}
