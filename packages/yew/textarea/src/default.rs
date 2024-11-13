use tailwind_fuse::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TextareaProps {
    // Global attributes
    #[prop_or_default]
    pub autocapitalize: Option<String>,
    #[prop_or_default]
    pub autocorrect: Option<String>,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub spellcheck: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    // Attributes from `textarea`
    #[prop_or_default]
    pub autocomplete: Option<String>,
    #[prop_or_default]
    pub cols: Option<String>,
    #[prop_or_default]
    pub dirname: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub form: Option<String>,
    #[prop_or_default]
    pub maxlength: Option<String>,
    #[prop_or_default]
    pub minlength: Option<String>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub pattern: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub rows: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,

    // Event handler attributes
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_change: Callback<Event>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_input: Callback<InputEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
}

#[function_component]
pub fn Textarea(props: &TextareaProps) -> Html {
    html! {
        <textarea
            ref={props.node_ref.clone()}

            autocapitalize={props.autocapitalize.clone()}
            autocorrect={props.autocorrect.clone()}
            autofocus={props.autofocus}
            class={tw_merge!(
                "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-base ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
                &props.class
            )}
            id={props.id.clone()}
            spellcheck={props.spellcheck.clone()}
            style={props.style.clone()}

            autocomplete={props.autocomplete.clone()}
            cols={props.cols.clone()}
            dirname={props.dirname.clone()}
            disabled={props.disabled}
            form={props.form.clone()}
            maxlength={props.maxlength.clone()}
            minlength={props.minlength.clone()}
            name={props.name.clone()}
            pattern={props.pattern.clone()}
            placeholder={props.placeholder.clone()}
            readonly={props.readonly}
            required={props.required}
            rows={props.rows.clone()}
            value={props.value.clone()}

            onblur={props.on_blur.clone()}
            onchange={props.on_change.clone()}
            onfocus={props.on_focus.clone()}
            oninput={props.on_input.clone()}
        />
    }
}
