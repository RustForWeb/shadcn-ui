use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
)]
pub struct TextareaClass {}

#[component]
pub fn Textarea(#[prop(into, optional)] class: MaybeSignal<String>) -> impl IntoView {
    let class = Memo::new(move |_| TextareaClass {}.with_class(class.get()));

    view! {
        <textarea class=class />
    }
}
