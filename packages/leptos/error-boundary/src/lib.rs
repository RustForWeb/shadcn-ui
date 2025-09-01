//! Production-Ready Error Handling for Leptos
//! 
//! This module provides simple, effective error handling for production applications.
//! It focuses on graceful degradation and user experience rather than complex error boundaries.

use leptos::prelude::*;
use std::panic::PanicInfo;

/// Simple error information for production use
#[derive(Clone, Debug)]
pub struct ErrorInfo {
    /// User-friendly error message
    pub message: String,
    /// Technical error details (for logging)
    pub technical_details: Option<String>,
}

/// Simple error fallback component
#[component]
pub fn ErrorFallback(
    #[prop(into)] error_info: ErrorInfo,
) -> impl IntoView {
    view! {
        <div class="error-fallback">
            <div class="error-content">
                <div class="error-icon">!</div>
                <h2 class="error-title">Something went wrong</h2>
                <p class="error-message">
                    {error_info.message}
                </p>
                <div class="error-actions">
                    <button 
                        class="error-retry"
                        on:click=move |_| {
                            // Simple page reload for now
                            if let Some(window) = web_sys::window() {
                                let _ = window.location().reload();
                            }
                        }
                    >
                        "Try Again"
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Simple error boundary wrapper
#[component]
pub fn ErrorBoundary(
    #[prop(into)] children: Children,
) -> impl IntoView {
    let (has_error, set_has_error) = signal(false);
    let (error_info, set_error_info) = signal(None::<ErrorInfo>);
    
    // Set up panic hook for production error handling
    std::panic::set_hook(Box::new(move |panic_info: &PanicInfo<'_>| {
        log::error!("Panic caught: {:?}", panic_info);
        
        let error = ErrorInfo {
            message: "An unexpected error occurred. Please try refreshing the page.".to_string(),
            technical_details: Some(format!("{:?}", panic_info)),
        };
        
        set_error_info.set(Some(error));
        set_has_error.set(true);
    }));
    
    // Render children or error fallback
    move || {
        if has_error.get() {
            if let Some(error) = error_info.get() {
                view! { <ErrorFallback error_info=error /> }.into_view()
            } else {
                view! { <ErrorFallback error_info=ErrorInfo { message: "An error occurred".to_string(), technical_details: None } /> }.into_view()
            }
        } else {
            children().into_view()
        }
    }
}

/// Hook for manual error handling
pub fn use_error_handler() -> (ReadSignal<bool>, WriteSignal<bool>, WriteSignal<Option<ErrorInfo>>) {
    let (has_error, set_has_error) = signal(false);
    let (error_info, set_error_info) = signal(None::<ErrorInfo>);
    
    (has_error, set_has_error, set_error_info)
}

/// Utility function to create user-friendly error messages
pub fn create_user_error(message: &str, technical: Option<&str>) -> ErrorInfo {
    ErrorInfo {
        message: message.to_string(),
        technical_details: technical.map(|s| s.to_string()),
    }
}

/// Utility function to handle errors gracefully
pub fn handle_error<T>(result: Result<T, impl std::fmt::Debug>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("Error occurred: {:?}", error);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_info_creation() {
        let error = ErrorInfo {
            message: "Test error".to_string(),
            technical_details: Some("Technical details".to_string()),
        };
        
        assert_eq!(error.message, "Test error");
        assert_eq!(error.technical_details, Some("Technical details".to_string()));
    }

    #[test]
    fn test_create_user_error() {
        let error = create_user_error("User message", Some("Technical"));
        assert_eq!(error.message, "User message");
        assert_eq!(error.technical_details, Some("Technical".to_string()));
    }

    #[test]
    fn test_handle_error() {
        let result: Result<i32, &str> = Ok(42);
        assert_eq!(handle_error(result), Some(42));
        
        let error_result: Result<i32, &str> = Err("Error");
        assert_eq!(handle_error(error_result), None);
    }
}
