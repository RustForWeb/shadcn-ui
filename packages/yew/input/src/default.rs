use tailwind_fuse::*;
use yew::prelude::*;
use yew_style::Style;

#[derive(PartialEq, Properties)]
pub struct InputProps {
    // Global attribnutes
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
    pub style: Style,

    // Attributes from `input`
    #[prop_or_default]
    pub accept: Option<String>,
    #[prop_or_default]
    pub alt: Option<String>,
    #[prop_or_default]
    pub autocomplete: Option<String>,
    #[prop_or_default]
    pub capture: Option<String>,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub dirname: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub form: Option<String>,
    #[prop_or_default]
    pub formaction: Option<String>,
    #[prop_or_default]
    pub formenctype: Option<String>,
    #[prop_or_default]
    pub formmethod: Option<String>,
    #[prop_or_default]
    pub formnovalidate: bool,
    #[prop_or_default]
    pub formtarget: Option<String>,
    #[prop_or_default]
    pub height: Option<String>,
    #[prop_or_default]
    pub list: Option<String>,
    #[prop_or_default]
    pub max: Option<String>,
    #[prop_or_default]
    pub maxlength: Option<String>,
    #[prop_or_default]
    pub min: Option<String>,
    #[prop_or_default]
    pub minlength: Option<String>,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub pattern: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub popovertarget: Option<String>,
    #[prop_or_default]
    pub popovertargetaction: Option<String>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub src: Option<String>,
    #[prop_or_default]
    pub step: Option<String>,
    #[prop_or_default]
    pub r#type: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub width: Option<String>,

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
pub fn Input(props: &InputProps) -> Html {
    html! {
        <input
            ref={props.node_ref.clone()}

            autocapitalize={props.autocapitalize.clone()}
            autocorrect={props.autocorrect.clone()}
            autofocus={props.autofocus}
            class={tw_merge!(
                "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                &props.class
            )}
            id={props.id.clone()}
            spellcheck={props.spellcheck.clone()}
            style={props.style.clone()}

            accept={props.accept.clone()}
            alt={props.alt.clone()}
            autocomplete={props.autocomplete.clone()}
            capture={props.capture.clone()}
            checked={props.checked}
            dirname={props.dirname.clone()}
            disabled={props.disabled}
            form={props.form.clone()}
            formaction={props.formaction.clone()}
            formenctype={props.formenctype.clone()}
            formmethod={props.formmethod.clone()}
            formnovalidate={props.formnovalidate}
            formtarget={props.formtarget.clone()}
            height={props.height.clone()}
            list={props.list.clone()}
            max={props.max.clone()}
            maxlength={props.maxlength.clone()}
            min={props.min.clone()}
            minlength={props.minlength.clone()}
            multiple={props.multiple}
            name={props.name.clone()}
            pattern={props.pattern.clone()}
            placeholder={props.placeholder.clone()}
            popovertarget={props.popovertarget.clone()}
            popovertargetaction={props.popovertargetaction.clone()}
            readonly={props.readonly}
            required={props.required}
            src={props.src.clone()}
            step={props.step.clone()}
            type={props.r#type.clone()}
            value={props.value.clone()}
            width={props.width.clone()}

            onblur={props.on_blur.clone()}
            onchange={props.on_change.clone()}
            onfocus={props.on_focus.clone()}
            oninput={props.on_input.clone()}
        />
    }
}
