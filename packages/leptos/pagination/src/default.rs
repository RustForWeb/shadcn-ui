use leptos::prelude::*;
use tailwind_fuse::tw_merge;
use lucide_leptos::{ChevronLeft, ChevronRight};

const PAGINATION_CLASS: &str = "mx-auto flex w-full justify-center";
const PAGINATION_CONTENT_CLASS: &str = "flex flex-row items-center gap-1";
const PAGINATION_ITEM_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2";
const PAGINATION_LINK_CLASS: &str = "gap-1 pl-2.5";
const PAGINATION_PREVIOUS_CLASS: &str = "gap-1 pr-2.5";
const PAGINATION_ELLIPSIS_CLASS: &str = "flex h-9 w-9 items-center justify-center";

#[component]
pub fn Pagination(
    #[prop(optional)] current_page: MaybeProp<usize>,
    #[prop(default = 1)] total_pages: usize,
    #[prop(optional)] on_page_change: Option<Callback<usize>>,
    #[prop(optional)] show_previous_next: MaybeProp<bool>,
    #[prop(optional)] show_first_last: MaybeProp<bool>,
    #[prop(optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let current = current_page.get().unwrap_or(1);
    let show_prev_next = show_previous_next.get().unwrap_or(true);
    let show_first_last_pages = show_first_last.get().unwrap_or(false);
    
    let handle_page_change = move |page: usize| {
        if let Some(on_page_change) = on_page_change {
            on_page_change.run(page);
        }
    };
    
    let get_visible_pages = move || -> Vec<Option<usize>> {
        let mut pages = Vec::new();
        
        if total_pages <= 7 {
            // Show all pages if 7 or fewer
            for i in 1..=total_pages {
                pages.push(Some(i));
            }
        } else {
            // Always show first page
            if show_first_last_pages {
                pages.push(Some(1));
            }
            
            // Calculate range around current page
            let start = if current <= 3 {
                1
            } else if current >= total_pages - 2 {
                total_pages - 4
            } else {
                current - 2
            };
            
            let end = if current <= 3 {
                5
            } else if current >= total_pages - 2 {
                total_pages
            } else {
                current + 2
            };
            
            // Add ellipsis before if needed
            if show_first_last_pages && start > 2 {
                pages.push(None); // Ellipsis
            }
            
            // Add pages in range
            let actual_start = if show_first_last_pages && start > 1 { start.max(2) } else { start };
            let actual_end = if show_first_last_pages && end < total_pages { end.min(total_pages - 1) } else { end };
            
            for i in actual_start..=actual_end {
                pages.push(Some(i));
            }
            
            // Add ellipsis after if needed
            if show_first_last_pages && end < total_pages - 1 {
                pages.push(None); // Ellipsis
            }
            
            // Always show last page
            if show_first_last_pages && !pages.contains(&Some(total_pages)) {
                pages.push(Some(total_pages));
            }
        }
        
        pages
    };
    
    let merged_class = tw_merge!(&format!("{} {}", 
        PAGINATION_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <nav 
            class={merged_class}
            role="navigation"
            aria-label="pagination"
        >
            <div class=PAGINATION_CONTENT_CLASS>
                // Previous button
                {if show_prev_next {
                    view! {
                        <PaginationItem>
                            <PaginationPrevious 
                                disabled={(current <= 1).into()}
                                on_click=Callback::new(move |_| if current > 1 { handle_page_change(current - 1) })
                            />
                        </PaginationItem>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
                
                // Page numbers
                {get_visible_pages().into_iter().map(|page_opt| {
                    match page_opt {
                        Some(page) => view! {
                            <PaginationItem>
                                <PaginationLink 
                                    _page=page.into()
                                    is_active={(page == current).into()}
                                    on_click=Callback::new(move |_| handle_page_change(page))
                                >
                                    {page.to_string()}
                                </PaginationLink>
                            </PaginationItem>
                        }.into_any(),
                        None => view! {
                            <PaginationItem>
                                <PaginationEllipsis />
                            </PaginationItem>
                        }.into_any(),
                    }
                }).collect::<Vec<_>>()}
                
                // Next button
                {if show_prev_next {
                    view! {
                        <PaginationItem>
                            <PaginationNext 
                                disabled={(current >= total_pages).into()}
                                on_click=Callback::new(move |_| if current < total_pages { handle_page_change(current + 1) })
                            />
                        </PaginationItem>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </div>
        </nav>
    }
}

#[component]
pub fn PaginationContent(
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        PAGINATION_CONTENT_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div class={merged_class}>
            {children()}
        </div>
    }
}

#[component]
pub fn PaginationItem(
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class={class.get().unwrap_or_default()}>
            {children()}
        </div>
    }
}

#[component]
pub fn PaginationLink(
    #[prop(optional)] _page: MaybeProp<usize>,
    #[prop(optional)] is_active: MaybeProp<bool>,
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let is_active_val = is_active.get().unwrap_or(false);
    let is_disabled = disabled.get().unwrap_or(false);
    
    let button_class = if is_active_val {
        tw_merge!(&format!("{} bg-primary text-primary-foreground hover:bg-primary/80", PAGINATION_ITEM_CLASS))
    } else {
        PAGINATION_ITEM_CLASS.to_string()
    };
    
    let merged_class = tw_merge!(&format!("{} {}", 
        button_class,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <button
            class={merged_class}
            disabled={is_disabled}
            on:click=move |_| {
                if !is_disabled {
                    if let Some(on_click) = on_click {
                        on_click.run(());
                    }
                }
            }
            aria-current={if is_active_val { "page" } else { "false" }}
        >
            {children()}
        </button>
    }
}

#[component]
pub fn PaginationPrevious(
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let is_disabled = disabled.get().unwrap_or(false);
    
    let merged_class = tw_merge!(&format!("{} {} {}", 
        PAGINATION_ITEM_CLASS,
        PAGINATION_PREVIOUS_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <button
            class={merged_class}
            disabled={is_disabled}
            on:click=move |_| {
                if !is_disabled {
                    if let Some(on_click) = on_click {
                        on_click.run(());
                    }
                }
            }
            aria-label="Go to previous page"
        >
            <ChevronLeft />
            {if let Some(children) = children {
                children().into_any()
            } else {
                view! { <span>"Previous"</span> }.into_any()
            }}
        </button>
    }
}

#[component]
pub fn PaginationNext(
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let is_disabled = disabled.get().unwrap_or(false);
    
    let merged_class = tw_merge!(&format!("{} {} {}", 
        PAGINATION_ITEM_CLASS,
        PAGINATION_LINK_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <button
            class={merged_class}
            disabled={is_disabled}
            on:click=move |_| {
                if !is_disabled {
                    if let Some(on_click) = on_click {
                        on_click.run(());
                    }
                }
            }
            aria-label="Go to next page"
        >
            {if let Some(children) = children {
                children().into_any()
            } else {
                view! { <span>"Next"</span> }.into_any()
            }}
            <ChevronRight />
        </button>
    }
}

#[component]
pub fn PaginationEllipsis(
    #[prop(optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        PAGINATION_ELLIPSIS_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <span 
            class={merged_class}
            aria-hidden="true"
        >
            <span class="h-4 w-4">...</span>
            <span class="sr-only">"More pages"</span>
        </span>
    }
}