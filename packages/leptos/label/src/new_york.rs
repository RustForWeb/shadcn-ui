use radix_leptos_label::Label as LabelPrimitive;
use shadcn_ui_leptos_utils::handlers::MaybeCallback;
use tailwind_fuse::*;
use leptos::{ev::MouseEvent, prelude::*};
use leptos_style::Style;

#[component]
pub fn Label(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    
  // node_ref needs to be implemented into LabelPrimitive. Or implement IntoProperty,
    //#[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] r#for: MaybeProp<String>,
    #[prop(into, optional)] on_mouse_down: MaybeCallback<MouseEvent>,
    #[prop(optional)] children: Option<Children>,

) -> impl IntoView {
    view! {
        <LabelPrimitive
            attr:class=move || tw_merge!("text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70", class.get())
            attr:style=style.get().to_string()
            attr:id=id.get()
            attr:r#for=r#for.get()
            on_mouse_down=on_mouse_down
        >
            {children.map(|children| children())}
        </LabelPrimitive>
    }
}