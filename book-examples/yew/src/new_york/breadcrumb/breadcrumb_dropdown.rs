use radix_yew_icons::{ChevronDownIcon, SlashIcon};
use yew::prelude::*;

use crate::new_york::components::ui::breadcrumb::{
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
                    <SlashIcon />
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
                    <ChevronDownIcon class="h-4 w-4" />
                </BreadcrumbItem>
                <BreadcrumbSeparator>
                    <SlashIcon />
                </BreadcrumbSeparator>
                <BreadcrumbItem>
                    <BreadcrumbPage>{"Breadcrumb"}</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
