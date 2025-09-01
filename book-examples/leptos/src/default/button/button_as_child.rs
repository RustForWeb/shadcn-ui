use leptos::prelude::*;

use crate::default::components::ui::button::{Button, ButtonChildProps, ButtonVariant, ButtonSize};

#[component]
pub fn ButtonAsChild() -> impl IntoView {
    let (click_count, set_click_count) = signal(0);

    let handle_click = Callback::new(move |_| {
        set_click_count.update(|count| *count += 1);
    });

    // Example 1: Button as a link
    let link_as_child = Callback::new(move |props: ButtonChildProps| {
        view! {
            <a
                href="#"
                class=props.class
                id=props.id
                style=props.style
                on:click=move |_| {
                    if let Some(onclick) = &props.onclick {
                        onclick.run(());
                    }
                }
            >
                "Link Button"
            </a>
        }
        .into_any()
    });

    // Example 2: Button as a div
    let div_as_child = Callback::new(move |props: ButtonChildProps| {
        view! {
            <div
                class=props.class
                id=props.id
                style=props.style
                on:click=move |_| {
                    if let Some(onclick) = &props.onclick {
                        onclick.run(());
                    }
                }
            >
                "Div Button"
            </div>
        }
        .into_any()
    });

    // Example 3: Button as a span
    let span_as_child = Callback::new(move |props: ButtonChildProps| {
        view! {
            <span
                class=props.class
                id=props.id
                style=props.style
                on:click=move |_| {
                    if let Some(onclick) = &props.onclick {
                        onclick.run(());
                    }
                }
            >
                "Span Button"
            </span>
        }
        .into_any()
    });

    view! {
        <div class="space-y-6 p-6">
            <div class="space-y-2">
                <h2 class="text-2xl font-bold">"Button as_child Examples"</h2>
                <p class="text-muted-foreground">
                    "Demonstrating polymorphic button rendering with as_child prop"
                </p>
                <p class="text-sm text-muted-foreground">
                    "Click count: " {move || click_count.get()}
                </p>
            </div>

            <div class="space-y-4">
                <div class="space-y-2">
                    <h3 class="text-lg font-semibold">"1. Button as Link"</h3>
                    <Button as_child=link_as_child on_click=handle_click.clone()>
                        "Link Button"
                    </Button>
                </div>

                <div class="space-y-2">
                    <h3 class="text-lg font-semibold">"2. Button as Div"</h3>
                    <Button as_child=div_as_child variant=ButtonVariant::Outline on_click=handle_click.clone()>
                        "Div Button"
                    </Button>
                </div>

                <div class="space-y-2">
                    <h3 class="text-lg font-semibold">"3. Button as Span"</h3>
                    <Button as_child=span_as_child variant=ButtonVariant::Secondary size=ButtonSize::Sm on_click=handle_click.clone()>
                        "Span Button"
                    </Button>
                </div>

                <div class="space-y-2">
                    <h3 class="text-lg font-semibold">"4. Regular Button (for comparison)"</h3>
                    <Button variant=ButtonVariant::Destructive on_click=handle_click.clone()>
                        "Regular Button"
                    </Button>
                </div>
            </div>

            <div class="space-y-2">
                <h3 class="text-lg font-semibold">"Usage Examples"</h3>
                <div class="bg-muted p-4 rounded-md">
                    <pre class="text-sm">
                        {r#"
// Button as a link
<Button as_child=link_callback>
    "Link Button"
</Button>

// Button as a div
<Button as_child=div_callback variant="outline">
    "Div Button"
</Button>

// Button as a span
<Button as_child=span_callback variant="secondary" size="sm">
    "Span Button"
</Button>
                        "#}
                    </pre>
                </div>
            </div>
        </div>
    }
}
