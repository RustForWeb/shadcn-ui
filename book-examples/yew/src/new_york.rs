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
        use self::alert::{render, AlertRoute};
        children.push(html! {
            <Switch<AlertRoute> render={render} />
        });
    }
    #[cfg(feature = "aspect-ratio")]
    {
        use self::aspect_ratio::{render, AspectRatioRoute};
        children.push(html! {
            <Switch<AspectRatioRoute> render={render} />
        });
    }
    #[cfg(feature = "avatar")]
    {
        use self::avatar::{render, AvatarRoute};
        children.push(html! {
            <Switch<AvatarRoute> render={render} />
        });
    }
    #[cfg(feature = "badge")]
    {
        use self::badge::{render, BadgeRoute};
        children.push(html! {
            <Switch<BadgeRoute> render={render} />
        });
    }
    #[cfg(feature = "breadcrumb")]
    {
        use self::breadcrumb::{render, BreadcrumbRoute};
        children.push(html! {
            <Switch<BreadcrumbRoute> render={render} />
        });
    }
    #[cfg(feature = "button")]
    {
        use self::button::{render, ButtonRoute};
        children.push(html! {
            <Switch<ButtonRoute> render={render} />
        });
    }
    #[cfg(feature = "card")]
    {
        use self::card::{render, CardRoute};
        children.push(html! {
            <Switch<CardRoute> render={render} />
        });
    }
    #[cfg(feature = "input")]
    {
        use self::input::{render, InputRoute};
        children.push(html! {
            <Switch<InputRoute> render={render} />
        });
    }
    #[cfg(feature = "label")]
    {
        use self::label::{render, LabelRoute};
        children.push(html! {
            <Switch<LabelRoute> render={render} />
        });
    }
    #[cfg(feature = "pagination")]
    {
        use self::pagination::{render, PaginationRoute};
        children.push(html! {
            <Switch<PaginationRoute> render={render} />
        });
    }
    #[cfg(feature = "separator")]
    {
        use self::separator::{render, SeparatorRoute};
        children.push(html! {
            <Switch<SeparatorRoute> render={render} />
        });
    }
    #[cfg(feature = "skeleton")]
    {
        use self::skeleton::{render, SkeletonRoute};
        children.push(html! {
            <Switch<SkeletonRoute> render={render} />
        });
    }
    #[cfg(feature = "switch")]
    {
        use self::switch::{render, SwitchRoute};
        children.push(html! {
            <Switch<SwitchRoute> render={render} />
        });
    }
    #[cfg(feature = "table")]
    {
        use self::table::{render, TableRoute};
        children.push(html! {
            <Switch<TableRoute> render={render} />
        });
    }
    #[cfg(feature = "textarea")]
    {
        use self::textarea::{render, TextareaRoute};
        children.push(html! {
            <Switch<TextareaRoute> render={render} />
        });
    }

    children.into_iter().collect()
}
