use yew::prelude::*;
use yew_lucide::{ChevronDown, Slash};

use crate::default::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};

#[function_component]
pub fn BreadcrumbDropdown() -> Html {
    html! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink href="/">{"Home"}</BreadcrumbLink>
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
                    {"Components"}
                    <ChevronDown class="h-4 w-4" />
                </BreadcrumbItem>
                <BreadcrumbSeparator>
                    <Slash />
                </BreadcrumbSeparator>
                <BreadcrumbItem>
                    <BreadcrumbPage>{"Breadcrumb"}</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
