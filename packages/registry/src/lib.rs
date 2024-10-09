pub mod registry_base_colors;
pub mod registry_blocks;
pub mod registry_charts;
pub mod registry_colors;
pub mod registry_examples;
pub mod registry_frameworks;
pub mod registry_hooks;
pub mod registry_lib;
pub mod registry_styles;
pub mod registry_themes;
pub mod registry_ui;
pub mod schema;

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::registry_blocks::BLOCKS;
use crate::registry_charts::CHARTS;
use crate::registry_examples::EXAMPLES;
use crate::registry_hooks::HOOKS;
use crate::registry_lib::LIB;
use crate::registry_themes::THEMES;
use crate::registry_ui::UI;
use crate::schema::{FrameworkName, Registry};

pub static REGISTRY: LazyLock<HashMap<FrameworkName, Registry>> = LazyLock::new(|| {
    let mut registry = HashMap::new();

    for map in [
        BLOCKS.clone(),
        CHARTS.clone(),
        EXAMPLES.clone(),
        HOOKS.clone(),
        LIB.clone(),
        THEMES.clone(),
        UI.clone(),
    ] {
        for (framework, entries) in map {
            registry
                .entry(framework)
                .or_insert_with(Vec::new)
                .extend(entries);
        }
    }

    registry
});
