use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
)]
pub struct InputClass {}

#[component]
pub fn Input(#[prop(into, optional)] class: MaybeSignal<String>) -> impl IntoView {
    let class = Memo::new(move |_| InputClass {}.with_class(class.get()));

    view! {
        <input class=class />
    }
}
