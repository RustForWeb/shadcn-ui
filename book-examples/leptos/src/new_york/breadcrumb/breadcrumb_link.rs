use leptos::prelude::*;

use crate::new_york::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbLinkChildProps, BreadcrumbList,
    BreadcrumbPage, BreadcrumbSeparator,
};

#[component]
pub fn BreadcrumbLinkDemo() -> impl IntoView {
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
                    <BreadcrumbLink
                        as_child={Callback::new(|BreadcrumbLinkChildProps {class, ..}| {
                            view! {
                                <a class={class} href="#/components">"Components"</a>
                            }
                            .into_any()
                        })}
                    />
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}