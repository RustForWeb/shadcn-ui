use leptos::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
)]
pub struct TextareaClass {}

#[component]
pub fn Textarea(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| TextareaClass {}.with_class(class.get()));

    view! {
        <textarea {..attributes} class=class />
    }
}
