use leptos::prelude::*;
use shadcn_ui_leptos_form::{Form, FormField, FormItem, FormLabel, FormControl, FormMessage, FormDescription};
use shadcn_ui_leptos_form::default::FormData;
use shadcn_ui_leptos_input::Input;
use shadcn_ui_leptos_button::Button;

#[component]
pub fn FormExample() -> impl IntoView {
    let (form_data, set_form_data) = signal(FormData::new());
    let (errors, set_errors) = signal(Vec::<(String, String)>::new());
    
    let handle_submit = Callback::new(move |data: FormData| {
        set_form_data.set(data.clone());
        
        // Simple validation
        let mut new_errors = Vec::new();
        
        if let Some(email) = data.get("email") {
            if email.is_empty() {
                new_errors.push(("email".to_string(), "Email is required".to_string()));
            } else if !email.contains('@') {
                new_errors.push(("email".to_string(), "Please enter a valid email".to_string()));
            }
        }
        
        if let Some(password) = data.get("password") {
            if password.is_empty() {
                new_errors.push(("password".to_string(), "Password is required".to_string()));
            } else if password.len() < 6 {
                new_errors.push(("password".to_string(), "Password must be at least 6 characters".to_string()));
            }
        }
        
        if let Some(name) = data.get("name") {
            if name.is_empty() {
                new_errors.push(("name".to_string(), "Name is required".to_string()));
            }
        }
        
        set_errors.set(new_errors);
        
        if errors.get().is_empty() {
            log::info!("Form submitted successfully!");
        }
    });
    
    let get_error = move |field: &str| {
        errors.get()
            .iter()
            .find(|(f, _)| f == field)
            .map(|(_, msg)| msg.clone())
    };
    
    view! {
        <div class="p-8 space-y-6">
            <div class="space-y-2">
                <h2 class="text-2xl font-bold">Form Example</h2>
                <p class="text-muted-foreground">
                    "A complete form with validation and accessibility features."
                </p>
            </div>
            
            <Form on_submit=handle_submit>
                <FormField name="name">
                    <FormItem>
                        <FormLabel for_field="name">"Name"</FormLabel>
                        <FormControl>
                            <Input
                                id="name"
                                placeholder="Enter your name"
                            />
                        </FormControl>
                        <FormMessage message=Signal::derive(move || get_error("name")) />
                    </FormItem>
                </FormField>
                
                <FormField name="email">
                    <FormItem>
                        <FormLabel for_field="email">"Email"</FormLabel>
                        <FormControl>
                            <Input
                                id="email"
                                input_type="email"
                                placeholder="Enter your email"
                            />
                        </FormControl>
                        <FormMessage message=Signal::derive(move || get_error("email")) />
                        <FormDescription>
                            "We'll never share your email with anyone else."
                        </FormDescription>
                    </FormItem>
                </FormField>
                
                <FormField name="password">
                    <FormItem>
                        <FormLabel for_field="password">"Password"</FormLabel>
                        <FormControl>
                            <Input
                                id="password"
                                input_type="password"
                                placeholder="Enter your password"
                            />
                        </FormControl>
                        <FormMessage message=Signal::derive(move || get_error("password")) />
                        <FormDescription>
                            "Password must be at least 6 characters long."
                        </FormDescription>
                    </FormItem>
                </FormField>
                
                <Button class="w-full">
                    "Submit"
                </Button>
            </Form>
            
            <div class=move || {
                if !form_data.get().fields.is_empty() {
                    "p-4 bg-muted rounded-md"
                } else {
                    "p-4 bg-muted rounded-md hidden"
                }
            }>
                <h3 class="text-lg font-semibold mb-2">"Submitted Data:"</h3>
                <pre class="text-sm">
                    {move || format!("{:#?}", form_data.get().fields)}
                </pre>
            </div>
            
            <div class="space-y-4">
                <h3 class="text-lg font-semibold">"Features Demonstrated:"</h3>
                <ul class="list-disc list-inside space-y-1 text-sm text-muted-foreground">
                    <li>"Form submission handling"</li>
                    <li>"Field validation with error messages"</li>
                    <li>"Accessible form labels and descriptions"</li>
                    <li>"Form data collection and processing"</li>
                    <li>"Responsive design"</li>
                </ul>
            </div>
        </div>
    }
}
