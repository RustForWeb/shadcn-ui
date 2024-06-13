use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RegistryItemType {
    #[serde(rename = "components:example")]
    Example,
    #[serde(rename = "components:component")]
    Component,
    #[serde(rename = "components:ui")]
    Ui,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryItem {
    pub name: String,
    pub dependencies: Option<Vec<String>>,
    pub dev_dependencies: Option<Vec<String>>,
    pub registry_dependencies: Option<Vec<String>>,
    pub files: Vec<String>,
    pub r#type: RegistryItemType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryItemWithContent {
    pub name: String,
    pub dependencies: Option<Vec<String>>,
    pub dev_dependencies: Option<Vec<String>>,
    pub registry_dependencies: Option<Vec<String>>,
    pub files: Vec<RegistryItemFile>,
    pub r#type: RegistryItemType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryItemFile {
    pub name: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryStyle {
    pub name: String,
    pub label: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryBaseColor {
    inline_colors: RegistryBaseColorThemes,
    css_vars: RegistryBaseColorThemes,
    inline_colors_template: String,
    css_vars_template: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryBaseColorThemes {
    pub light: HashMap<String, String>,
    pub dark: HashMap<String, String>,
}
