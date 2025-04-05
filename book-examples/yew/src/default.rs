mod components;

#[cfg(feature = "alert")]
mod alert;
#[cfg(feature = "aspect-ratio")]
mod aspect_ratio;
#[cfg(feature = "avatar")]
mod avatar;
#[cfg(feature = "badge")]
mod badge;
#[cfg(feature = "breadcrumb")]
mod breadcrumb;
#[cfg(feature = "button")]
mod button;
#[cfg(feature = "card")]
mod card;
#[cfg(feature = "input")]
mod input;
#[cfg(feature = "label")]
mod label;
#[cfg(feature = "pagination")]
mod pagination;
#[cfg(feature = "separator")]
mod separator;
#[cfg(feature = "skeleton")]
mod skeleton;
#[cfg(feature = "switch")]
mod switch;
#[cfg(feature = "table")]
mod table;
#[cfg(feature = "textarea")]
mod textarea;

use yew::prelude::*;
use yew_router::prelude::*;

pub fn render() -> Html {
    let mut children: Vec<Html> = vec![];

    #[cfg(feature = "alert")]
    {
        use self::alert::{AlertRoute, render};
        children.push(html! {
            <Switch<AlertRoute> render={render} />
        });
    }
    #[cfg(feature = "aspect-ratio")]
    {
        use self::aspect_ratio::{AspectRatioRoute, render};
        children.push(html! {
            <Switch<AspectRatioRoute> render={render} />
        });
    }
    #[cfg(feature = "avatar")]
    {
        use self::avatar::{AvatarRoute, render};
        children.push(html! {
            <Switch<AvatarRoute> render={render} />
        });
    }
    #[cfg(feature = "badge")]
    {
        use self::badge::{BadgeRoute, render};
        children.push(html! {
            <Switch<BadgeRoute> render={render} />
        });
    }
    #[cfg(feature = "breadcrumb")]
    {
        use self::breadcrumb::{BreadcrumbRoute, render};
        children.push(html! {
            <Switch<BreadcrumbRoute> render={render} />
        });
    }
    #[cfg(feature = "button")]
    {
        use self::button::{ButtonRoute, render};
        children.push(html! {
            <Switch<ButtonRoute> render={render} />
        });
    }
    #[cfg(feature = "card")]
    {
        use self::card::{CardRoute, render};
        children.push(html! {
            <Switch<CardRoute> render={render} />
        });
    }
    #[cfg(feature = "input")]
    {
        use self::input::{InputRoute, render};
        children.push(html! {
            <Switch<InputRoute> render={render} />
        });
    }
    #[cfg(feature = "label")]
    {
        use self::label::{LabelRoute, render};
        children.push(html! {
            <Switch<LabelRoute> render={render} />
        });
    }
    #[cfg(feature = "pagination")]
    {
        use self::pagination::{PaginationRoute, render};
        children.push(html! {
            <Switch<PaginationRoute> render={render} />
        });
    }
    #[cfg(feature = "separator")]
    {
        use self::separator::{SeparatorRoute, render};
        children.push(html! {
            <Switch<SeparatorRoute> render={render} />
        });
    }
    #[cfg(feature = "skeleton")]
    {
        use self::skeleton::{SkeletonRoute, render};
        children.push(html! {
            <Switch<SkeletonRoute> render={render} />
        });
    }
    #[cfg(feature = "switch")]
    {
        use self::switch::{SwitchRoute, render};
        children.push(html! {
            <Switch<SwitchRoute> render={render} />
        });
    }
    #[cfg(feature = "table")]
    {
        use self::table::{TableRoute, render};
        children.push(html! {
            <Switch<TableRoute> render={render} />
        });
    }
    #[cfg(feature = "textarea")]
    {
        use self::textarea::{TextareaRoute, render};
        children.push(html! {
            <Switch<TextareaRoute> render={render} />
        });
    }

    children.into_iter().collect()
}
