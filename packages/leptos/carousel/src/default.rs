use leptos::prelude::*;
use web_sys::MouseEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum CarouselOrientation {
    Horizontal,
    Vertical,
}

impl Default for CarouselOrientation {
    fn default() -> Self {
        CarouselOrientation::Horizontal
    }
}

#[derive(Debug, Clone)]
pub struct CarouselApi {
    pub scroll_to: Callback<usize>,
    pub scroll_prev: Callback<()>,
    pub scroll_next: Callback<()>,
    pub can_scroll_prev: Signal<bool>,
    pub can_scroll_next: Signal<bool>,
}

#[component]
pub fn Carousel(
    #[prop(into, optional)] orientation: Signal<CarouselOrientation>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let selected_index: RwSignal<usize> = RwSignal::new(0);
    let total_items: RwSignal<usize> = RwSignal::new(0);
    
    let can_scroll_prev = Signal::derive(move || selected_index.get() > 0);
    let can_scroll_next = Signal::derive(move || selected_index.get() < total_items.get().saturating_sub(1_usize));
    
    let scroll_to = Callback::new(move |index: usize| {
        selected_index.set(index);
    });
    
    let scroll_prev = Callback::new(move |_| {
        let current = selected_index.get();
        if current > 0 {
            selected_index.set(current - 1);
        }
    });
    
    let scroll_next = Callback::new(move |_| {
        let current = selected_index.get();
        let total = total_items.get();
        if current < total.saturating_sub(1_usize) {
            selected_index.set(current + 1);
        }
    });
    
    let api = CarouselApi {
        scroll_to,
        scroll_prev,
        scroll_next,
        can_scroll_prev,
        can_scroll_next,
    };
    
    provide_context(orientation);
    provide_context(api);
    provide_context(selected_index);
    provide_context(total_items);
    
    let computed_class = Signal::derive(move || {
        format!("relative w-full {}", class.get().unwrap_or_default())
    });
    
    view! {
        <div class=computed_class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CarouselContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let orientation = expect_context::<Signal<CarouselOrientation>>();
    let total_items = expect_context::<RwSignal<usize>>();
    
    // Update total items count when children change without moving children into the Effect
    let has_any_children = children.is_some();
    Effect::new(move |_| {
        if has_any_children {
            // This is a simplified approach - in a real implementation you'd count actual children
            total_items.set(1);
        }
    });
    
    let computed_class = Signal::derive(move || {
        let position_class = match orientation.get() {
            CarouselOrientation::Horizontal => "flex",
            CarouselOrientation::Vertical => "flex flex-col",
        };
        
        format!("{} w-full {}", position_class, class.get().unwrap_or_default())
    });
    
    let rendered_children = children.map(|c| c());
    view! {
        <div class=computed_class>
            {rendered_children}
        </div>
    }
}

#[component]
pub fn CarouselItem(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let orientation = expect_context::<Signal<CarouselOrientation>>();
    
    let computed_class = Signal::derive(move || {
        let position_class = match orientation.get() {
            CarouselOrientation::Horizontal => "min-w-0 shrink-0 grow-0 basis-full",
            CarouselOrientation::Vertical => "min-h-0 shrink-0 grow-0 basis-full",
        };
        
        format!("{} {}", position_class, class.get().unwrap_or_default())
    });
    
    view! {
        <div class=computed_class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CarouselPrevious(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] variant: MaybeProp<String>,
    #[prop(into, optional)] size: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let orientation = expect_context::<Signal<CarouselOrientation>>();
    let api = expect_context::<CarouselApi>();

    let handle_click = move |_: MouseEvent| {
        api.scroll_prev.run(());
        if let Some(callback) = &on_click {
            callback.run(());
        }
    };

    let computed_class = Signal::derive(move || {
        let position_class = match orientation.get() {
            CarouselOrientation::Horizontal => "absolute h-8 w-8 rounded-full left-12 top-1/2 -translate-y-1/2",
            CarouselOrientation::Vertical => "absolute h-8 w-8 rounded-full left-1/2 top-12 -translate-x-1/2 -rotate-90",
        };
        
        format!(
            "{} inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground {}",
            position_class,
            class.get().unwrap_or_default()
        )
    });

    view! {
        <button
            class=computed_class
            disabled=move || !api.can_scroll_prev.get()
            aria-label="Previous slide"
            on:click=handle_click
        >
            {children.map(|c| c())}
            <svg
                class="h-4 w-4"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="m15 18-6-6 6-6"/>
            </svg>
            <span class="sr-only">"Previous slide"</span>
        </button>
    }
}

#[component]
pub fn CarouselNext(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] variant: MaybeProp<String>,
    #[prop(into, optional)] size: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let orientation = expect_context::<Signal<CarouselOrientation>>();
    let api = expect_context::<CarouselApi>();

    let handle_click = move |_: MouseEvent| {
        api.scroll_next.run(());
        if let Some(callback) = &on_click {
            callback.run(());
        }
    };

    let computed_class = Signal::derive(move || {
        let position_class = match orientation.get() {
            CarouselOrientation::Horizontal => "absolute h-8 w-8 rounded-full right-12 top-1/2 -translate-y-1/2",
            CarouselOrientation::Vertical => "absolute h-8 w-8 rounded-full left-1/2 bottom-12 -translate-x-1/2 rotate-90",
        };
        
        format!(
            "{} inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground {}",
            position_class,
            class.get().unwrap_or_default()
        )
    });

    view! {
        <button
            class=computed_class
            disabled=move || !api.can_scroll_next.get()
            aria-label="Next slide"
            on:click=handle_click
        >
            {children.map(|c| c())}
            <svg
                class="h-4 w-4"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="m9 18 6-6-6-6"/>
            </svg>
            <span class="sr-only">"Next slide"</span>
        </button>
    }
}