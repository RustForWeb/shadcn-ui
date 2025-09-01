use leptos::prelude::*;
use tailwind_fuse::tw_merge;

const BREADCRUMB_CLASS: &str = "";
const BREADCRUMB_LIST_CLASS: &str = "flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5";
const BREADCRUMB_ITEM_CLASS: &str = "inline-flex items-center gap-1.5";
const BREADCRUMB_LINK_CLASS: &str = "transition-colors hover:text-foreground";
const BREADCRUMB_PAGE_CLASS: &str = "font-normal text-foreground";
const BREADCRUMB_SEPARATOR_CLASS: &str = "[&>svg]:size-3.5";
const BREADCRUMB_ELLIPSIS_CLASS: &str = "flex h-9 w-9 items-center justify-center";

#[component]
pub fn Breadcrumb(
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        BREADCRUMB_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <nav 
            aria-label="breadcrumb" 
            class={merged_class}
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn BreadcrumbList(
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        BREADCRUMB_LIST_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <ol class={merged_class}>
            {children()}
        </ol>
    }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        BREADCRUMB_ITEM_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <li class={merged_class}>
            {children()}
        </li>
    }
}

#[component]
pub fn BreadcrumbLink(
    #[prop(optional)] href: MaybeProp<String>,
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] as_child: MaybeProp<bool>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        BREADCRUMB_LINK_CLASS,
        class.get().unwrap_or_default()
    ));
    
    let is_as_child = as_child.get().unwrap_or(false);
    
    if is_as_child {
        // When as_child is true, render children directly with class applied
        children()
    } else {
        view! {
            <a 
                href={href.get()}
                class={merged_class}
            >
                {children()}
            </a>
        }.into_any()
    }
}

#[component]
pub fn BreadcrumbPage(
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        BREADCRUMB_PAGE_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <span 
            role="link"
            aria-disabled="true"
            aria-current="page"
            class={merged_class}
        >
            {children()}
        </span>
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        BREADCRUMB_SEPARATOR_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <li role="presentation" aria-hidden="true" class={merged_class}>
            {if let Some(children) = children {
                children().into_any()
            } else {
                view! { 
                    <svg 
                        width="15" 
                        height="15" 
                        viewBox="0 0 15 15" 
                        fill="none" 
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path 
                            d="m5.5 4.5 3 3-3 3" 
                            stroke="currentColor" 
                            stroke-width="1" 
                            stroke-linecap="round" 
                            stroke-linejoin="round"
                        />
                    </svg> 
                }.into_any()
            }}
        </li>
    }
}

#[component]
pub fn BreadcrumbEllipsis(
    #[prop(optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        BREADCRUMB_ELLIPSIS_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <span 
            role="presentation" 
            aria-hidden="true"
            class={merged_class}
        >
            <svg 
                width="15" 
                height="15" 
                viewBox="0 0 15 15" 
                fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <path 
                    d="M3 6.5a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM7.5 6.5a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM13.5 8a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0Z" 
                    fill="currentColor"
                />
            </svg>
            <span class="sr-only">"More"</span>
        </span>
    }
}