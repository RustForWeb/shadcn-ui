use leptos::prelude::*;
use lucide_leptos::{ChevronDown, Slash};

use crate::new_york::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};

#[component]
pub fn BreadcrumbDropdownDemo() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink href="#/">"Home"</BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator>
                    <Slash />
                </BreadcrumbSeparator>
                <BreadcrumbItem>
                // TODO
                    // <DropdownMenu>
                    //     <DropdownMenuTrigger class="flex items-center gap-1">
                    //         {"Components"}
                    //         <ChevronDown class="h-4 w-4" />
                    //     </DropdownMenuTrigger>
                    //     <DropdownMenuContent align="start">
                    //         <DropdownMenuItem>{"Documentation"}</DropdownMenuItem>
                    //         <DropdownMenuItem>{"Themes"}</DropdownMenuItem>
                    //         <DropdownMenuItem>{"GitHub"}</DropdownMenuItem>
                    //     </DropdownMenuContent>
                    // </DropdownMenu>
                    "Components"
                    <ChevronDown size=16 />
                </BreadcrumbItem>
                <BreadcrumbSeparator>
                    <Slash />
                </BreadcrumbSeparator>
                <BreadcrumbItem>
                    <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
