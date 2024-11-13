use yew::prelude::*;
use yew_router::prelude::*;

use crate::new_york::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbLinkChildProps,
    BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/docs/components")]
    Components,
}

#[function_component]
pub fn BreadcrumbEllipsisDemo() -> Html {
    html! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink
                        as_child={Callback::from(|BreadcrumbLinkChildProps {class, ..}| html! {
                            <Link<Route> classes={classes!(class)} to={Route::Home}>{"Home"}</Link<Route>>
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
                        as_child={Callback::from(|BreadcrumbLinkChildProps {class, ..}| html! {
                            <Link<Route> classes={classes!(class)} to={Route::Home}>{"Components"}</Link<Route>>
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
