pub mod registry_base_colors;
pub mod registry_blocks;
pub mod registry_charts;
pub mod registry_colors;
pub mod registry_examples;
pub mod registry_hooks;
pub mod registry_lib;
pub mod registry_styles;
pub mod registry_themes;
pub mod registry_ui;
pub mod schema;

use std::sync::LazyLock;

use crate::registry_blocks::BLOCKS;
use crate::registry_charts::CHARTS;
use crate::registry_examples::EXAMPLES;
use crate::registry_hooks::HOOKS;
use crate::registry_lib::LIB;
use crate::registry_themes::THEMES;
use crate::registry_ui::UI;
use crate::schema::Registry;

pub static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
    let mut registry = vec![];

    registry.extend(BLOCKS.clone());
    registry.extend(CHARTS.clone());
    registry.extend(EXAMPLES.clone());
    registry.extend(HOOKS.clone());
    registry.extend(LIB.clone());
    registry.extend(THEMES.clone());
    registry.extend(UI.clone());

    registry
});
