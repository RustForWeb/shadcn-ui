use lucide_yew::Slash;
use yew::prelude::*;

use crate::default::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};

#[function_component]
pub fn BreadcrumbSeparatorDemo() -> Html {
    html! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink href="#/">{"Home"}</BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator>
                    <Slash />
                </BreadcrumbSeparator>
                <BreadcrumbItem>
                    <BreadcrumbLink href="#/components">{"Components"}</BreadcrumbLink>
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
