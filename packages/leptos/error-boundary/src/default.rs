//! Default implementation of the Error Handling system

pub use super::*;

// Re-export the main components for easy access
pub use super::{
    ErrorBoundary,
    ErrorFallback,
    ErrorInfo,
    use_error_handler,
    create_user_error,
    handle_error,
};

// Default styling classes for the error fallback
pub const ERROR_FALLBACK_CLASSES: &str = "error-fallback";
pub const ERROR_CONTENT_CLASSES: &str = "error-content";
pub const ERROR_ICON_CLASSES: &str = "error-icon";
pub const ERROR_TITLE_CLASSES: &str = "error-title";
pub const ERROR_MESSAGE_CLASSES: &str = "error-message";
pub const ERROR_ACTIONS_CLASSES: &str = "error-actions";
pub const ERROR_RETRY_CLASSES: &str = "error-retry";

// Default error messages
pub const DEFAULT_ERROR_TITLE: &str = "Something went wrong";
pub const DEFAULT_ERROR_MESSAGE: &str = "An unexpected error occurred. Please try refreshing the page.";
pub const DEFAULT_RETRY_TEXT: &str = "Try Again";

// Default error icons
pub const DEFAULT_ERROR_ICON: &str = "!";
pub const DEFAULT_ERROR_ICON_ALT: &str = "Error icon";

// Error handling configuration
pub const DEFAULT_LOG_ERRORS: bool = true;
pub const DEFAULT_SHOW_TECHNICAL_DETAILS: bool = false; // Usually too technical for users

// Error fallback styling constants
pub const ERROR_FALLBACK_STYLES: &str = r#"
    .error-fallback {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: 200px;
        padding: 2rem;
        background-color: #fef2f2;
        border: 1px solid #fecaca;
        border-radius: 0.5rem;
        margin: 1rem 0;
    }
    
    .error-content {
        text-align: center;
        max-width: 500px;
    }
    
    .error-icon {
        font-size: 3rem;
        margin-bottom: 1rem;
        color: #dc2626;
        font-weight: bold;
    }
    
    .error-title {
        font-size: 1.5rem;
        font-weight: 600;
        color: #dc2626;
        margin-bottom: 0.5rem;
    }
    
    .error-message {
        color: #6b7280;
        margin-bottom: 1.5rem;
        line-height: 1.5;
    }
    
    .error-actions {
        display: flex;
        justify-content: center;
    }
    
    .error-retry {
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 0.375rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        background-color: #2563eb;
        color: white;
    }
    
    .error-retry:hover {
        background-color: #1d4ed8;
    }
"#;
