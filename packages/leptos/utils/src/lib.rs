//! Leptos port of [shadcn/ui utils](https://ui.shadcn.com/docs/installation/manual#add-a-cn-helper).
//!
//! Utility for Tailwind CSS classes.
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/) for more documenation.

pub mod default;
pub mod new_york;


pub mod handlers {
    use leptos::prelude::{Callable, Callback};
    pub fn generate_handler<T>(callback: Option<Callback<T>>) -> impl FnMut(T)
    where
        T: 'static,
    {
        move |event: T| {
            if let Some(callback) = callback {
                callback.run(event);
            }
        }
    }
}