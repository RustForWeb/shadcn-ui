use leptos::prelude::*;
use js_sys::Date;

const CALENDAR_GRID_CLASS: &str = "grid w-full grid-cols-7 gap-px";
const CALENDAR_HEADER_CLASS: &str = "grid w-full grid-cols-7 gap-px";
const CALENDAR_HEADER_CELL_CLASS: &str = "flex h-9 w-full items-center justify-center text-xs font-medium";
const CALENDAR_ROW_CLASS: &str = "grid w-full grid-cols-7 gap-px";
const CALENDAR_CELL_CLASS: &str = "relative p-0 text-center text-sm focus-within:relative focus-within:z-20 [&:has([aria-selected])]:bg-accent first:[&:has([aria-selected])]:rounded-l-md last:[&:has([aria-selected])]:rounded-r-md";
const CALENDAR_DAY_CLASS: &str = "h-9 w-9 p-0 font-normal aria-selected:opacity-100";
const CALENDAR_DAY_SELECTED_CLASS: &str = "bg-primary text-primary-foreground hover:bg-primary hover:text-primary-foreground focus:bg-primary focus:text-primary-foreground";
const CALENDAR_DAY_TODAY_CLASS: &str = "bg-accent text-accent-foreground";
const CALENDAR_DAY_DISABLED_CLASS: &str = "text-muted-foreground opacity-50";
const CALENDAR_DAY_HIDDEN_CLASS: &str = "invisible";

#[derive(Debug, Clone, PartialEq)]
pub struct CalendarDate {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl CalendarDate {
    pub fn new(year: u32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    pub fn get_js_date(&self) -> Date {
        Date::new_with_year_month_day(self.year, (self.month - 1) as i32, self.day as i32)
    }
}

fn get_days_in_month(year: u32, month: u32) -> u32 {
    let date = Date::new_with_year_month_day(year, (month - 1) as i32, 1);
    let next_month = Date::new_with_year_month_day(year, month as i32, 1);
    ((next_month.get_time() - date.get_time()) / (1000.0 * 60.0 * 60.0 * 24.0)) as u32
}

fn get_first_day_of_month(year: u32, month: u32) -> u32 {
    let date = Date::new_with_year_month_day(year, (month - 1) as i32, 1);
    date.get_day() as u32
}

#[component]
pub fn Calendar(
    #[prop(into, optional)] mode: Signal<String>,
    #[prop(into, optional)] selected: RwSignal<Option<CalendarDate>>,
    #[prop(into, optional)] on_select: Option<Callback<CalendarDate>>,
    #[prop(into, optional)] disabled: Signal<Vec<CalendarDate>>,
    #[prop(into, optional)] initial_focus: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let today = CalendarDate::new(
        Date::new_0().get_full_year() as u32,
        (Date::new_0().get_month() + 1) as u32,
        Date::new_0().get_date() as u32,
    );
    
    let current_month = RwSignal::new((today.year, today.month));
    let disabled_dates = disabled;
    
    let handle_day_click = {
        let selected = selected.clone();
        let on_select = on_select.clone();
        move |date: CalendarDate| {
            selected.set(Some(date.clone()));
            if let Some(callback) = &on_select {
                callback.run(date);
            }
        }
    };
    
    let computed_class = Signal::derive(move || {
        format!("w-full {}", class.get().unwrap_or_default())
    });
    
    view! {
        <div class=computed_class>
            <div class="space-y-4">
                <div class="flex items-center justify-between">
                    <button
                        class="h-7 w-7 rounded-md border border-input bg-background p-0 opacity-50 hover:opacity-100"
                        on:click=move |_| {
                            let (year, month) = current_month.get();
                            if month == 1 {
                                current_month.set((year - 1, 12));
                            } else {
                                current_month.set((year, month - 1));
                            }
                        }
                    >
                        <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="m15 18-6-6 6-6"/>
                        </svg>
                    </button>
                    <div class="text-sm font-medium">
                        {move || {
                            let (year, month) = current_month.get();
                            format!("{} {}", month, year)
                        }}
                    </div>
                    <button
                        class="h-7 w-7 rounded-md border border-input bg-background p-0 opacity-50 hover:opacity-100"
                        on:click=move |_| {
                            let (year, month) = current_month.get();
                            if month == 12 {
                                current_month.set((year + 1, 1));
                            } else {
                                current_month.set((year, month + 1));
                            }
                        }
                    >
                        <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="m9 18 6-6-6-6"/>
                        </svg>
                    </button>
                </div>
                <div class="space-y-2">
                    <div class=format!("{}", CALENDAR_HEADER_CLASS)>
                        {vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"].into_iter().map(|day| {
                            view! {
                                <div class=CALENDAR_HEADER_CELL_CLASS>
                                    {day}
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                    <div class=format!("{}", CALENDAR_GRID_CLASS)>
                        {move || {
                            let (year, month) = current_month.get();
                            let days_in_month = get_days_in_month(year, month);
                            let first_day = get_first_day_of_month(year, month);
                            let selected = selected.get();
                            
                            let mut weeks: Vec<AnyView> = Vec::new();
                            let mut current_week: Vec<AnyView> = Vec::new();
                            
                            // Add empty cells for days before the first day of month
                            for _ in 0..first_day {
                                current_week.push(view! {
                                    <div class=CALENDAR_CELL_CLASS>
                                        <div class=format!("{} {}", CALENDAR_DAY_CLASS, CALENDAR_DAY_HIDDEN_CLASS)>
                                        </div>
                                    </div>
                                }.into_any());
                            }
                            
                            // Add days of the month
                            for day in 1..=days_in_month {
                                let date = CalendarDate::new(year, month, day);
                                let is_today = date == today;
                                let is_selected = selected.as_ref().map(|s| *s == date).unwrap_or(false);
                                let is_disabled = disabled_dates.get().contains(&date);
                                
                                let day_classes = if is_selected {
                                    format!("{} {}", CALENDAR_DAY_CLASS, CALENDAR_DAY_SELECTED_CLASS)
                                } else if is_today {
                                    format!("{} {}", CALENDAR_DAY_CLASS, CALENDAR_DAY_TODAY_CLASS)
                                } else if is_disabled {
                                    format!("{} {}", CALENDAR_DAY_CLASS, CALENDAR_DAY_DISABLED_CLASS)
                                } else {
                                    CALENDAR_DAY_CLASS.to_string()
                                };
                                
                                let date_for_click = date.clone();
                                current_week.push(view! {
                                    <div class=CALENDAR_CELL_CLASS>
                                        <div 
                                            class={day_classes}
                                            aria-selected={is_selected}
                                            data-today={is_today}
                                            aria-disabled={is_disabled}
                                            on:click=move |_| {
                                                if !is_disabled {
                                                    handle_day_click(date_for_click.clone());
                                                }
                                            }
                                            role="button"
                                            tabindex="0"
                                        >
                                            {day}
                                        </div>
                                    </div>
                                }.into_any());
                                
                                if current_week.len() == 7 {
                                    let row_items = current_week.drain(..).collect::<Vec<_>>();
                                    weeks.push(view! {
                                        <div class=CALENDAR_ROW_CLASS>
                                            {row_items}
                                        </div>
                                    }.into_any());
                                }
                            }
                            
                            // Fill the last week if needed
                            while current_week.len() < 7 && !current_week.is_empty() {
                                current_week.push(view! {
                                    <div class=CALENDAR_CELL_CLASS>
                                        <div class=format!("{} {}", CALENDAR_DAY_CLASS, CALENDAR_DAY_HIDDEN_CLASS)>
                                        </div>
                                    </div>
                                }.into_any());
                            }
                            
                            if !current_week.is_empty() {
                                let row_items = current_week;
                                weeks.push(view! {
                                    <div class=CALENDAR_ROW_CLASS>
                                        {row_items}
                                    </div>
                                }.into_any());
                            }
                            
                            weeks
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}