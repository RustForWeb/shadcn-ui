mod components;

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
