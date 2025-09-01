//! Core component generation logic.

use crate::{ComponentConfig, Framework};
use anyhow::Result;
use std::path::Path;

pub struct Generator;

impl Generator {
    pub fn generate_component_files(config: &ComponentConfig, output_dir: &Path) -> Result<()> {
        // Create component directory structure
        let component_dir = output_dir.join(&config.name);
        std::fs::create_dir_all(&component_dir)?;
        std::fs::create_dir_all(component_dir.join("src"))?;
        
        // Generate Cargo.toml
        let cargo_toml = Self::generate_cargo_toml(config)?;
        std::fs::write(component_dir.join("Cargo.toml"), cargo_toml)?;
        
        // Generate lib.rs
        let lib_rs = Self::generate_lib_rs(config)?;
        std::fs::write(component_dir.join("src").join("lib.rs"), lib_rs)?;
        
        // Generate theme variants
        for theme in &config.theme_variants {
            let theme_file = Self::generate_theme_component(config, theme)?;
            std::fs::write(
                component_dir.join("src").join(format!("{}.rs", theme)), 
                theme_file
            )?;
        }
        
        Ok(())
    }
    
    fn generate_cargo_toml(config: &ComponentConfig) -> Result<String> {
        let framework_name = match config.framework {
            Framework::Leptos => "leptos",
            Framework::Yew => "yew", 
            Framework::Dioxus => "dioxus",
        };
        
        Ok(format!(
            r#"[package]
name = "shadcn-ui-{}-{}"
description = "{} port of shadcn/ui {}."
homepage = "https://shadcn-ui.rustforweb.org/components/{}.html"
publish = false

authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
version.workspace = true

[dependencies]
tailwind_fuse.workspace = true
{}.workspace = true
web-sys.workspace = true

[dev-dependencies]
shadcn-ui-test-utils = {{ path = "../../test-utils", features = ["{}-testing"] }}
wasm-bindgen-test = {{ workspace = true }}
"#,
            framework_name,
            config.name,
            framework_name.chars().next().unwrap().to_uppercase().chain(framework_name.chars().skip(1)).collect::<String>(),
            config.name.chars().next().unwrap().to_uppercase().chain(config.name.chars().skip(1)).collect::<String>(),
            config.name,
            framework_name,
            framework_name
        ))
    }
    
    fn generate_lib_rs(config: &ComponentConfig) -> Result<String> {
        let component_title = config.name.chars().next().unwrap().to_uppercase().chain(config.name.chars().skip(1)).collect::<String>();
        let framework_title = match config.framework {
            Framework::Leptos => "Leptos",
            Framework::Yew => "Yew",
            Framework::Dioxus => "Dioxus",
        };
        
        Ok(format!(
            r#"//! {} port of [shadcn/ui {}](https://ui.shadcn.com/docs/components/{}).
//!
//! Component description here.
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/components/{}.html) for more documentation.

pub mod default;
pub mod new_york;

// Re-export the components for easy access
pub use default::*;

#[cfg(feature = "new_york")]
pub use new_york as {};
"#,
            framework_title,
            component_title,
            config.name,
            config.name,
            config.name
        ))
    }
    
    fn generate_theme_component(config: &ComponentConfig, theme: &str) -> Result<String> {
        // This would use the template engine to generate framework-specific component code
        // For now, return a placeholder
        Ok(format!(
            "// {} theme variant for {} component\n// Implementation goes here\n",
            theme, config.name
        ))
    }
}