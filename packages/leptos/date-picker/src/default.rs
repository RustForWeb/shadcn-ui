use leptos::prelude::*;
use tailwind_fuse::tw_merge;
use shadcn_ui_leptos_calendar::{Calendar as CalendarComponent, CalendarDate};
use shadcn_ui_leptos_button::{Button, ButtonVariant};

const DATE_PICKER_CLASS: &str = "w-full";
const DATE_PICKER_TRIGGER_CLASS: &str = "w-full justify-start text-left font-normal";
const DATE_PICKER_PLACEHOLDER_CLASS: &str = "text-muted-foreground";

#[component]
pub fn DatePicker(
    #[prop(optional)] selected: MaybeProp<CalendarDate>,
    #[prop(optional)] on_select: Option<Callback<CalendarDate>>,
    #[prop(optional)] disabled: MaybeProp<Vec<CalendarDate>>,
    #[prop(optional)] placeholder: MaybeProp<String>,
    #[prop(optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let is_open = RwSignal::new(false);
    let selected_date = RwSignal::new(selected.get());
    
    // Update selected date when prop changes
    Effect::new(move |_| {
        if let Some(new_selected) = selected.get() {
            selected_date.set(Some(new_selected));
        }
    });
    
    let handle_select = move |date: CalendarDate| {
        selected_date.set(Some(date.clone()));
        is_open.set(false);
        if let Some(on_select) = on_select {
            on_select.run(date);
        }
    };
    
    let format_date = |date: &CalendarDate| -> String {
        let months = [
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"
        ];
        format!("{} {}, {}", 
            months[(date.month - 1) as usize], 
            date.day, 
            date.year
        )
    };
    
    let merged_class = tw_merge!(&format!("{} {}", 
        DATE_PICKER_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div class={merged_class}>
            <Button 
                variant=ButtonVariant::Outline
                class={tw_merge!(&DATE_PICKER_TRIGGER_CLASS)}
                on:click=move |_| is_open.set(!is_open.get())
            >
                <svg class="mr-2 h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
                    <line x1="16" y1="2" x2="16" y2="6"></line>
                    <line x1="8" y1="2" x2="8" y2="6"></line>
                    <line x1="3" y1="10" x2="21" y2="10"></line>
                </svg>
                {move || {
                    if let Some(date) = selected_date.get() {
                        format_date(&date)
                    } else {
                        placeholder.get().unwrap_or_else(|| "Pick a date".to_string())
                    }
                }}
            </Button>
            {move || if is_open.get() {
                view! {
                    <div class="mt-2 w-auto p-0 border rounded-md bg-background">
                        <CalendarComponent
                            selected=selected_date
                            on_select=Callback::new(move |date: CalendarDate| {
                                selected_date.set(Some(date.clone()));
                                is_open.set(false);
                                if let Some(cb) = on_select.clone() {
                                    cb.run(date);
                                }
                            })
                            disabled=disabled.get().unwrap_or_default()
                        />
                    </div>
                }.into_any()
            } else { view! {}.into_any() }}
        </div>
    }
}

#[component]
pub fn DatePickerWithRange(
    #[prop(optional)] from: MaybeProp<CalendarDate>,
    #[prop(optional)] to: MaybeProp<CalendarDate>,
    #[prop(optional)] on_select: Option<Callback<(Option<CalendarDate>, Option<CalendarDate>)>>,
    #[prop(optional)] disabled: MaybeProp<Vec<CalendarDate>>,
    #[prop(optional)] placeholder: MaybeProp<String>,
    #[prop(optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let is_open = RwSignal::new(false);
    let range_start = RwSignal::new(from.get());
    let range_end = RwSignal::new(to.get());
    let selecting_end = RwSignal::new(false);
    
    // Update range when props change
    Effect::new(move |_| {
        if let Some(new_from) = from.get() {
            range_start.set(Some(new_from));
        }
    });
    
    Effect::new(move |_| {
        if let Some(new_to) = to.get() {
            range_end.set(Some(new_to));
        }
    });
    
    let handle_select = move |date: CalendarDate| {
        if !selecting_end.get() {
            // First selection - set start date
            range_start.set(Some(date.clone()));
            range_end.set(None);
            selecting_end.set(true);
        } else {
            // Second selection - set end date
            let start = range_start.get();
            if let Some(ref start_date) = start {
                // Ensure end is after start using tuple comparison
                if (date.year, date.month, date.day) >= (start_date.year, start_date.month, start_date.day)
                {
                    range_end.set(Some(date.clone()));
                } else {
                    // If selected date is before start, make it the new start
                    range_start.set(Some(date.clone()));
                    range_end.set(start.clone());
                }
            }
            selecting_end.set(false);
            is_open.set(false);
        }
        
        if let Some(on_select) = on_select {
            on_select.run((range_start.get(), range_end.get()));
        }
    };
    
    let format_date = |date: &CalendarDate| -> String {
        let months = [
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"
        ];
        format!("{} {}, {}", months[(date.month - 1) as usize], date.day, date.year)
    };

    let format_date_range = move || -> String {
        let start = range_start.get();
        let end = range_end.get();
        
        match (start, end) {
            (Some(start_date), Some(end_date)) => {
                format!("{} - {}", format_date(&start_date), format_date(&end_date))
            },
            (Some(start_date), None) => {
                format!("{} - ", format_date(&start_date))
            },
            _ => placeholder.get().unwrap_or_else(|| "Pick a date range".to_string())
        }
    };
    
    let merged_class = tw_merge!(&format!("{} {}", 
        DATE_PICKER_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div class={merged_class}>
            <Button 
                variant=ButtonVariant::Outline
                class={tw_merge!(&DATE_PICKER_TRIGGER_CLASS)}
                on:click=move |_| is_open.set(!is_open.get())
            >
                <svg class="mr-2 h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
                    <line x1="16" y1="2" x2="16" y2="6"></line>
                    <line x1="8" y1="2" x2="8" y2="6"></line>
                    <line x1="3" y1="10" x2="21" y2="10"></line>
                </svg>
                <span class={
                    move || if range_start.get().is_none() { 
                        DATE_PICKER_PLACEHOLDER_CLASS 
                    } else { 
                        "" 
                    }
                }>
                    {format_date_range}
                </span>
            </Button>
            {move || if is_open.get() {
                view! {
                    <div class="mt-2 w-auto p-0 border rounded-md bg-background">
                        <CalendarComponent
                            selected=range_start
                            on_select=Callback::new(move |date: CalendarDate| {
                                handle_select(date);
                            })
                            disabled=disabled.get().unwrap_or_default()
                        />
                    </div>
                }.into_any()
            } else { view! {}.into_any() }}
        </div>
    }
}