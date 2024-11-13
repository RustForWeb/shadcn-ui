use yew::prelude::*;

use crate::new_york::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};

#[function_component]
pub fn BreadcrumbDemo() -> Html {
    html! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink href="#/">{"Home"}</BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    // TODO
                    // <DropdownMenu>
                    //     <DropdownMenuTrigger class="flex items-center gap-1">
                    //         <BreadcrumbEllipsis class="h-4 w-4" />
                    //         <span class="sr-only">{"Toggle menu"}</span>
                    //     </DropdownMenuTrigger>
                    //     <DropdownMenuContent align="start">
                    //         <DropdownMenuItem>{"Documentation"}</DropdownMenuItem>
                    //         <DropdownMenuItem>{"Themes"}</DropdownMenuItem>
                    //         <DropdownMenuItem>{"GitHub"}</DropdownMenuItem>
                    //     </DropdownMenuContent>
                    // </DropdownMenu>
                    <BreadcrumbEllipsis class="h-4 w-4" />
                    <span class="sr-only">{"Toggle menu"}</span>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbLink href="#/docs/components">{"Components"}</BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>{"Breadcrumb"}</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
