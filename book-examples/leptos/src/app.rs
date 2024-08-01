use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let mut views: Vec<View> = vec![];

    #[cfg(feature = "alert")]
    {
        use crate::alert::AlertDemo;
        views.push(view! {
            <AlertDemo />
        });
    }
    #[cfg(feature = "alert-destructive")]
    {
        use crate::alert_destructive::AlertDestructive;
        views.push(view! {
            <AlertDestructive />
        });
    }
    #[cfg(feature = "button")]
    {
        use crate::button::ButtonDemo;
        views.push(view! {
            <ButtonDemo />
        });
    }
    #[cfg(feature = "button-secondary")]
    {
        use crate::button_secondary::ButtonSecondary;
        views.push(view! {
            <ButtonSecondary />
        });
    }
    #[cfg(feature = "button-destructive")]
    {
        use crate::button_destructive::ButtonDestructive;
        views.push(view! {
            <ButtonDestructive />
        });
    }
    #[cfg(feature = "button-outline")]
    {
        use crate::button_outline::ButtonOutline;
        views.push(view! {
            <ButtonOutline />
        });
    }
    #[cfg(feature = "button-ghost")]
    {
        use crate::button_ghost::ButtonGhost;
        views.push(view! {
            <ButtonGhost />
        });
    }
    #[cfg(feature = "button-link")]
    {
        use crate::button_link::ButtonLink;
        views.push(view! {
            <ButtonLink />
        });
    }
    #[cfg(feature = "button-icon")]
    {
        use crate::button_icon::ButtonIcon;
        views.push(view! {
            <ButtonIcon />
        });
    }
    #[cfg(feature = "button-with-icon")]
    {
        use crate::button_with_icon::ButtonWithIcon;
        views.push(view! {
            <ButtonWithIcon />
        });
    }
    #[cfg(feature = "button-loading")]
    {
        use crate::button_loading::ButtonLoading;
        views.push(view! {
            <ButtonLoading />
        });
    }
    #[cfg(feature = "button-as-child")]
    {
        use crate::button_as_child::ButtonAsChild;
        views.push(view! {
            <ButtonAsChild />
        });
    }

    // TODO: add style selector (provide value as context?)

    view! {
        <div class="flex min-h-[350px] w-full justify-center p-10 items-center">
            {views.into_view()}
        </div>
    }
}
