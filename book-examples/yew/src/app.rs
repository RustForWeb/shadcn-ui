use std::str::FromStr;

use web_sys::window;
use yew::prelude::*;

pub enum Style {
    Default,
    NewYork,
}

impl FromStr for Style {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Style::Default),
            "new-york" => Ok(Style::NewYork),
            _ => Err("Unknown style."),
        }
    }
}

#[function_component]
pub fn App() -> Html {
    let style: Style = window()
        .expect("Window should exist.")
        .location()
        .hash()
        .map(|hash| {
            hash.strip_prefix('#')
                .map(|hash| hash.to_string())
                .unwrap_or(hash)
        })
        .unwrap_or_default()
        .parse()
        .unwrap_or(Style::Default);

    let mut children: Vec<Html> = vec![];

    #[cfg(feature = "button")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::button::ButtonDemo /> },
            Style::NewYork => html! { <crate::new_york::button::ButtonDemo /> },
        });
    }
    #[cfg(feature = "button-secondary")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::button_secondary::ButtonSecondary /> },
            Style::NewYork => html! { <crate::new_york::button_secondary::ButtonSecondary /> },
        });
    }
    #[cfg(feature = "button-destructive")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::button_destructive::ButtonDestructive /> },
            Style::NewYork => html! { <crate::new_york::button_destructive::ButtonDestructive /> },
        });
    }
    #[cfg(feature = "button-outline")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::button_outline::ButtonOutline /> },
            Style::NewYork => html! { <crate::new_york::button_outline::ButtonOutline /> },
        });
    }
    #[cfg(feature = "button-ghost")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::button_ghost::ButtonGhost /> },
            Style::NewYork => html! { <crate::new_york::button_ghost::ButtonGhost /> },
        });
    }
    #[cfg(feature = "button-link")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::button_link::ButtonLink /> },
            Style::NewYork => html! { <crate::new_york::button_link::ButtonLink /> },
        });
    }
    #[cfg(feature = "button-icon")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::button_icon::ButtonIcon /> },
            Style::NewYork => html! { <crate::new_york::button_icon::ButtonIcon /> },
        });
    }
    #[cfg(feature = "button-with-icon")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::button_with_icon::ButtonWithIcon /> },
            Style::NewYork => html! { <crate::new_york::button_with_icon::ButtonWithIcon /> },
        });
    }
    #[cfg(feature = "button-loading")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::button_loading::ButtonLoading /> },
            Style::NewYork => html! { <crate::new_york::button_loading::ButtonLoading /> },
        });
    }
    #[cfg(feature = "button-as-child")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::button_as_child::ButtonAsChild /> },
            Style::NewYork => html! { <crate::new_york::button_as_child::ButtonAsChild /> },
        });
    }
    #[cfg(feature = "skeleton")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::skeleton::SkeletonDemo /> },
            Style::NewYork => html! { <crate::new_york::skeleton::SkeletonDemo /> },
        });
    }
    #[cfg(feature = "skeleton-card")]
    {
        children.push(match style {
            Style::Default => html! { <crate::default::skeleton_card::SkeletonCard /> },
            Style::NewYork => html! { <crate::new_york::skeleton_card::SkeletonCard /> },
        });
    }

    // TODO: add style selector (provide value as context?)

    html! {
        <div class="flex min-h-[350px] w-full justify-center p-10 items-center">
            {children}
        </div>
    }
}
