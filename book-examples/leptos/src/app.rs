use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let mut views: Vec<AnyView> = vec![];

    #[cfg(feature = "alert")]
    {
        use crate::alert::AlertDemo;
        views.push(
            view! {
                <AlertDemo />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "alert-destructive")]
    {
        use crate::alert_destructive::AlertDestructive;
        views.push(
            view! {
                <AlertDestructive />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "button")]
    {
        use crate::button::ButtonDemo;
        views.push(
            view! {
                <ButtonDemo />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "button-secondary")]
    {
        use crate::button_secondary::ButtonSecondary;
        views.push(
            view! {
                <ButtonSecondary />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "button-destructive")]
    {
        use crate::button_destructive::ButtonDestructive;
        views.push(
            view! {
                <ButtonDestructive />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "button-outline")]
    {
        use crate::button_outline::ButtonOutline;
        views.push(
            view! {
                <ButtonOutline />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "button-ghost")]
    {
        use crate::button_ghost::ButtonGhost;
        views.push(
            view! {
                <ButtonGhost />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "button-link")]
    {
        use crate::button_link::ButtonLink;
        views.push(
            view! {
                <ButtonLink />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "button-icon")]
    {
        use crate::button_icon::ButtonIcon;
        views.push(
            view! {
                <ButtonIcon />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "button-with-icon")]
    {
        use crate::button_with_icon::ButtonWithIcon;
        views.push(
            view! {
                <ButtonWithIcon />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "button-loading")]
    {
        use crate::button_loading::ButtonLoading;
        views.push(
            view! {
                <ButtonLoading />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "button-as-child")]
    {
        use crate::button_as_child::ButtonAsChild;
        views.push(
            view! {
                <ButtonAsChild />
            }
            .into_any(),
        );
    }

    // TODO: add style selector (provide value as context?)

    view! {
        <div class="flex min-h-[350px] w-full justify-center p-10 items-center">
            {views.into_view()}
        </div>
    }
}
