use tailwind_fuse::*;
use yew::prelude::*;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground hover:bg-destructive/90")]
    Destructive,
    #[tw(class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground")]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonSize {
    #[tw(default, class = "h-10 px-4 py-2")]
    Default,
    #[tw(class = "h-9 rounded-md px-3")]
    Sm,
    #[tw(class = "h-11 rounded-md px-8")]
    Lg,
    #[tw(class = "h-10 w-10")]
    Icon,
}

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,

    // Attributes from `button`
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub command: Option<String>,
    #[prop_or_default]
    pub commandfor: Option<String>,
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
    pub name: Option<String>,
    #[prop_or_default]
    pub popovertarget: Option<String>,
    #[prop_or_default]
    pub popovertargetaction: Option<String>,
    #[prop_or_default]
    pub r#type: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<ButtonChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct ButtonChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: String,
    pub style: Option<String>,

    pub autofocus: bool,
    pub command: Option<String>,
    pub commandfor: Option<String>,
    pub disabled: bool,
    pub form: Option<String>,
    pub formaction: Option<String>,
    pub formenctype: Option<String>,
    pub formmethod: Option<String>,
    pub formnovalidate: bool,
    pub formtarget: Option<String>,
    pub name: Option<String>,
    pub popovertarget: Option<String>,
    pub popovertargetaction: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<String>,
    pub on_click: Callback<MouseEvent>,
}

impl ButtonChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <button
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}

                autofocus={self.autofocus}
                command={self.command}
                commandfor={self.commandfor}
                disabled={self.disabled}
                form={self.form}
                formaction={self.formaction}
                formenctype={self.formenctype}
                formmethod={self.formmethod}
                formnovalidate={self.formnovalidate}
                formtarget={self.formtarget}
                name={self.name}
                popovertarget={self.popovertarget}
                popovertargetaction={self.popovertargetaction}
                type={self.r#type}
                value={self.value}
                onclick={self.on_click}
            >
                {children}
            </button>
        }
    }
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let class = use_memo(
        (props.variant, props.size, props.class.clone()),
        |(variant, size, class)| {
            ButtonClass {
                variant: *variant,
                size: *size,
            }
            .with_class(class.clone().unwrap_or_default())
        },
    );

    let child_props = ButtonChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: (*class).clone(),
        style: props.style.clone(),

        autofocus: props.autofocus,
        command: props.command.clone(),
        commandfor: props.commandfor.clone(),
        disabled: props.disabled,
        form: props.form.clone(),
        formaction: props.formaction.clone(),
        formenctype: props.formenctype.clone(),
        formmethod: props.formmethod.clone(),
        formnovalidate: props.formnovalidate,
        formtarget: props.formtarget.clone(),
        name: props.name.clone(),
        popovertarget: props.popovertarget.clone(),
        popovertargetaction: props.popovertargetaction.clone(),
        r#type: props.r#type.clone(),
        value: props.value.clone(),
        on_click: props.on_click.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
