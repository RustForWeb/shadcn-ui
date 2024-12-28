//! Leptos port of [shadcn/ui utils](https://ui.shadcn.com/docs/installation/manual#add-a-cn-helper).
//!
//! Utility for Tailwind CSS classes.
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/) for more documenation.

pub mod default;
pub mod new_york;

pub mod handlers {
    use leptos::prelude::{Callable, Callback};

    // Define an enum to handle both cases
    #[derive(Default)]
    pub enum MaybeCallback<T: 'static> {
        Some(Callback<T>),
        #[default]
        None,
    }

    // Implement a convenience trait for conversion from Option
    impl<T> From<Option<Callback<T>>> for MaybeCallback<T> {
        fn from(option: Option<Callback<T>>) -> Self {
            match option {
                Some(callback) => MaybeCallback::Some(callback),
                None => MaybeCallback::None,
            }
        }
    }

    // Implement a convenience trait for direct Callback
    impl<T> From<Callback<T>> for MaybeCallback<T> {
        fn from(callback: Callback<T>) -> Self {
            MaybeCallback::Some(callback)
        }
    }

    // Define the function to accept MaybeCallback
    pub fn generate_handler<T>(callback: impl Into<MaybeCallback<T>>) -> impl FnMut(T)
    where
        T: 'static,
    {
        let callback = callback.into();
        move |event: T| {
            if let MaybeCallback::Some(ref callback) = callback {
                callback.run(event);
            }
        }
    }

    pub struct Handler;

    impl Handler {
        pub fn from<T>(callback: MaybeCallback<T>) -> impl FnMut(T)
        where
            T: 'static,
        {
            move |event: T| {
                if let MaybeCallback::Some(callback) = callback {
                    callback.run(event);
                }
            }
        }
    }
}