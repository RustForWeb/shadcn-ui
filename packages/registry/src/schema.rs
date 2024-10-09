use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Light,
    Dark,
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mode::Light => "light",
                Mode::Dark => "dark",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Style {
    Default,
    NewYork,
}

impl Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Style::Default => "default",
                Style::NewYork => "new-york",
            }
        )
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockChunk {
    pub name: String,
    pub description: String,
    // pub component: Any,
    pub file: String,
    pub code: Option<String>,
    pub container: Option<BlockChunkContainer>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockChunkContainer {
    pub class_name: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum RegistryItemType {
    #[serde(rename = "registry:block")]
    Block,
    #[serde(rename = "registry:component")]
    Component,
    #[serde(rename = "registry:example")]
    Example,
    #[serde(rename = "registry:hook")]
    Hook,
    #[serde(rename = "registry:lib")]
    Lib,
    #[serde(rename = "registry:page")]
    Page,
    #[serde(rename = "registry:style")]
    Style,
    #[serde(rename = "registry:theme")]
    Theme,
    #[serde(rename = "registry:ui")]
    Ui,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryItemFile {
    pub path: String,
    pub content: Option<String>,
    pub r#type: RegistryItemType,
    pub target: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryItemTailwind {
    pub config: RegistryItemTailwindConfig,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryItemTailwindConfig {
    pub content: Option<Vec<String>>,
    // pub theme: Option<HashMap<String, Any>>,
    pub plugins: Option<Vec<String>>,
}

pub type RegistryItemCssVars = HashMap<Mode, HashMap<String, String>>;

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryEntry {
    pub name: String,
    pub r#type: RegistryItemType,
    pub description: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub dev_dependencies: Option<Vec<String>>,
    pub registry_dependencies: Option<Vec<String>>,
    pub files: Option<Vec<RegistryItemFile>>,
    pub tailwind: Option<RegistryItemTailwind>,
    pub css_vars: Option<RegistryItemCssVars>,
    pub source: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub chunks: Option<Vec<BlockChunk>>,
    pub docs: Option<String>,
}

pub type Registry = Vec<RegistryEntry>;

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub name: String,
    pub r#type: RegistryItemType,
    pub description: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub dev_dependencies: Option<Vec<String>>,
    pub registry_dependencies: Option<Vec<String>>,
    pub files: Option<Vec<RegistryItemFile>>,
    pub tailwind: Option<RegistryItemTailwind>,
    pub css_vars: Option<RegistryItemCssVars>,
    pub source: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub chunks: Option<Vec<BlockChunk>>,
    pub docs: Option<String>,
    pub style: Style,
    // pub component: Any,
    pub container: Option<BlockContainer>,
    pub code: String,
    pub highlighted_code: String,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockContainer {
    pub height: Option<String>,
    pub class_name: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FrameworkName {
    Dioxus,
    Leptos,
    Yew,
}

impl Display for FrameworkName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FrameworkName::Dioxus => "dioxus",
                FrameworkName::Leptos => "leptos",
                FrameworkName::Yew => "yew",
            }
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Framework {
    pub name: FrameworkName,
    pub label: String,
    pub detect_dependencies: Vec<String>,
}
