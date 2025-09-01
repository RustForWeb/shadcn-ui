//! Component generation utilities for shadcn-ui.
//!
//! This package provides tools for generating consistent component implementations
//! across different Rust web frameworks (Leptos, Yew, Dioxus).

pub mod generator;
pub mod templates;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Framework types supported by the generator
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Framework {
    Leptos,
    Yew,
    Dioxus,
}

/// Component generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentConfig {
    pub name: String,
    pub framework: Framework,
    pub theme_variants: Vec<String>,
    pub props: HashMap<String, PropConfig>,
    pub dependencies: Vec<String>,
}

/// Property configuration for component props
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropConfig {
    pub prop_type: String,
    pub optional: bool,
    pub default_value: Option<String>,
    pub description: Option<String>,
}

/// Main component generator
pub struct ComponentGenerator {
    pub template_engine: handlebars::Handlebars<'static>,
}

impl ComponentGenerator {
    pub fn new() -> Result<Self> {
        let mut handlebars = handlebars::Handlebars::new();
        
        // Register built-in templates
        handlebars.register_template_string("leptos_component", include_str!("templates/leptos_component.hbs"))?;
        handlebars.register_template_string("yew_component", include_str!("templates/yew_component.hbs"))?;
        handlebars.register_template_string("lib_rs", include_str!("templates/lib_rs.hbs"))?;
        
        Ok(Self {
            template_engine: handlebars,
        })
    }
    
    pub fn generate_component(&self, config: &ComponentConfig) -> Result<String> {
        let template_name = match config.framework {
            Framework::Leptos => "leptos_component",
            Framework::Yew => "yew_component",
            Framework::Dioxus => "dioxus_component",
        };
        
        self.template_engine.render(template_name, config)
            .map_err(Into::into)
    }
}

impl Default for ComponentGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to initialize component generator")
    }
}