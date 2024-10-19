use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let mut children: Vec<Html> = vec![];

    #[cfg(feature = "button")]
    {
        use crate::button::ButtonDemo;
        children.push(html! {
            <ButtonDemo />
        });
    }
    #[cfg(feature = "button-secondary")]
    {
        use crate::button_secondary::ButtonSecondary;
        children.push(html! {
            <ButtonSecondary />
        });
    }
    #[cfg(feature = "button-destructive")]
    {
        use crate::button_destructive::ButtonDestructive;
        children.push(html! {
            <ButtonDestructive />
        });
    }
    #[cfg(feature = "button-outline")]
    {
        use crate::button_outline::ButtonOutline;
        children.push(html! {
            <ButtonOutline />
        });
    }
    #[cfg(feature = "button-ghost")]
    {
        use crate::button_ghost::ButtonGhost;
        children.push(html! {
            <ButtonGhost />
        });
    }
    #[cfg(feature = "button-link")]
    {
        use crate::button_link::ButtonLink;
        children.push(html! {
            <ButtonLink />
        });
    }
    #[cfg(feature = "button-icon")]
    {
        use crate::button_icon::ButtonIcon;
        children.push(html! {
            <ButtonIcon />
        });
    }
    #[cfg(feature = "button-with-icon")]
    {
        use crate::button_with_icon::ButtonWithIcon;
        children.push(html! {
            <ButtonWithIcon />
        });
    }
    #[cfg(feature = "button-loading")]
    {
        use crate::button_loading::ButtonLoading;
        children.push(html! {
            <ButtonLoading />
        });
    }
    #[cfg(feature = "button-as-child")]
    {
        use crate::button_as_child::ButtonAsChild;
        children.push(html! {
            <ButtonAsChild />
        });
    }
    #[cfg(feature = "skeleton")]
    {
        use crate::skeleton::SkeletonDemo;
        children.push(html! {
            <SkeletonDemo />
        });
    }
    #[cfg(feature = "skeleton-card")]
    {
        use crate::skeleton_card::SkeletonCard;
        children.push(html! {
            <SkeletonCard />
        });
    }

    // TODO: add style selector (provide value as context?)

    html! {
        <div class="flex min-h-[350px] w-full justify-center p-10 items-center">
            {children}
        </div>
    }
}
