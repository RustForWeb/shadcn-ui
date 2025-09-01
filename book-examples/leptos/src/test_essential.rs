//! Test file to verify essential components build without icon dependencies

use leptos::*;

// Test that we can import essential components
#[cfg(feature = "button")]
use shadcn_ui_leptos_button::Button;

#[cfg(feature = "input")]
use shadcn_ui_leptos_input::Input;

#[cfg(feature = "label")]
use shadcn_ui_leptos_label::Label;

#[cfg(feature = "card")]
use shadcn_ui_leptos_card::{Card, CardContent, CardHeader, CardTitle};

#[cfg(feature = "separator")]
use shadcn_ui_leptos_separator::Separator;

#[component]
pub fn TestEssentialComponents() -> impl IntoView {
    view! {
        <div class="test-essential">
            <h1>"Testing Essential Components"</h1>
            
            #[cfg(feature = "button")]
            <div class="component-test">
                <h2>"Button Component"</h2>
                <Button>"Test Button"</Button>
            </div>
            
            #[cfg(feature = "input")]
            <div class="component-test">
                <h2>"Input Component"</h2>
                <Input placeholder="Test input" />
            </div>
            
            #[cfg(feature = "label")]
            <div class="component-test">
                <h2>"Label Component"</h2>
                <Label>"Test Label"</Label>
            </div>
            
            #[cfg(feature = "card")]
            <div class="component-test">
                <h2>"Card Component"</h2>
                <Card>
                    <CardHeader>
                        <CardTitle>"Test Card"</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <p>"This is a test card content"</p>
                    </CardContent>
                </Card>
            </div>
            
            #[cfg(feature = "separator")]
            <div class="component-test">
                <h2>"Separator Component"</h2>
                <Separator />
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_essential_components_import() {
        // This test verifies that all essential components can be imported
        // without any icon dependencies
        let _component = TestEssentialComponents;
        assert!(true); // If we get here, the imports worked
    }
}
