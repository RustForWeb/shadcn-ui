mod components;

#[cfg(feature = "alert")]
mod alert;
#[cfg(feature = "avatar")]
mod avatar;
#[cfg(feature = "button")]
mod button;
#[cfg(feature = "pagination")]
mod pagination;
#[cfg(feature = "skeleton")]
mod skeleton;

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
    #[cfg(feature = "avatar")]
    {
        use self::avatar::{render, AvatarRoute};
        children.push(html! {
            <Switch<AvatarRoute> render={render} />
        });
    }
    #[cfg(feature = "button")]
    {
        use self::button::{render, ButtonRoute};
        children.push(html! {
            <Switch<ButtonRoute> render={render} />
        });
    }
    #[cfg(feature = "pagination")]
    {
        use self::pagination::{render, PaginationRoute};
        children.push(html! {
            <Switch<PaginationRoute> render={render} />
        });
    }
    #[cfg(feature = "skeleton")]
    {
        use self::skeleton::{render, SkeletonRoute};
        children.push(html! {
            <Switch<SkeletonRoute> render={render} />
        });
    }

    children.into_iter().collect()
}
