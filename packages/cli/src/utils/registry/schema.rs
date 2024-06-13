use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum RegistryItemType {
    Example,
    Component,
    Ui,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistryItem {
    pub name: String,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub registry_dependencies: Vec<String>,
    pub files: Vec<String>,
    pub r#type: RegistryItemType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistryItemWithContent {
    pub name: String,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub registry_dependencies: Vec<String>,
    pub files: Vec<RegistryItemFile>,
    pub r#type: RegistryItemType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistryItemFile {
    pub name: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistryStyle {
    pub name: String,
    pub label: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistryBaseColor {
    inline_colors: RegistryBaseColorThemes,
    css_vars: RegistryBaseColorThemes,
    inline_colors_template: String,
    css_vars_template: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistryBaseColorThemes {
    pub light: String,
    pub dark: String,
}
