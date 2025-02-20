use leptos::prelude::*;

use crate::default::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbLinkChildProps,
    BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};

#[component]
pub fn BreadcrumbEllipsisDemo() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink
                        as_child={Callback::new(|BreadcrumbLinkChildProps {class, ..}| {
                            view! {
                                <a class={class} href="#/">"Home"</a>
                            }
                            .into_any()
                        })}
                    />
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbEllipsis />
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbLink
                        as_child={Callback::new(|BreadcrumbLinkChildProps {class, ..}| {
                            view! {
                                <a class={class} href="#/docs/components">"Components"</a>
                            }
                            .into_any()
                        })}
                    />
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>{"Breadcrumb"}</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
