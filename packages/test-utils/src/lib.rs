//! Testing utilities for shadcn-ui components across frameworks.
//!
//! This package provides shared testing infrastructure for validating component
//! implementations across Leptos and Yew frameworks.

pub mod component_tester;
pub mod theme_validator;
pub mod parity_checker;
pub mod visual_regression;
pub mod leptos_testing;
pub mod test_templates;

use std::collections::HashMap;

/// Framework types for cross-framework testing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Framework {
    Leptos,
    Yew,
    Dioxus,
}

/// Theme variants supported by components
#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Default,
    NewYork,
}

/// Test execution results
#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub message: String,
    pub details: HashMap<String, String>,
}

/// Cross-framework parity test results
#[derive(Debug, Clone)]
pub struct ParityResult {
    pub frameworks_match: bool,
    pub differences: Vec<String>,
    pub score: f64, // 0.0-1.0 compatibility score
}

impl TestResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            passed: true,
            message: message.into(),
            details: HashMap::new(),
        }
    }
    
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            passed: false,
            message: message.into(),
            details: HashMap::new(),
        }
    }
    
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }
}

impl ParityResult {
    pub fn perfect_match() -> Self {
        Self {
            frameworks_match: true,
            differences: vec![],
            score: 1.0,
        }
    }
    
    pub fn with_differences(differences: Vec<String>) -> Self {
        let score = if differences.is_empty() {
            1.0
        } else {
            // Simple scoring: reduce by 0.1 per difference, minimum 0.0
            (1.0 - (differences.len() as f64 * 0.1)).max(0.0)
        };
        
        Self {
            frameworks_match: differences.is_empty(),
            differences,
            score,
        }
    }
}