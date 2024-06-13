use std::error::Error;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

use crate::utils::get_config::Config;
use crate::utils::registry::schema::{RegistryItem, RegistryItemWithContent, RegistryStyle};

use super::RegistryBaseColor;

const BASE_URL: &str = "https://ui.shadcn.com";

pub async fn get_registry_index() -> Result<Vec<RegistryItem>, Box<dyn Error>> {
    Ok(
        fetch_registry::<Vec<RegistryItem>>(vec!["index.json".into()])
            .await?
            .remove(0),
    )
}

pub async fn get_registry_styles() -> Result<Vec<RegistryStyle>, Box<dyn Error>> {
    Ok(
        fetch_registry::<Vec<RegistryStyle>>(vec!["styles/index.json".into()])
            .await?
            .remove(0),
    )
}

pub fn get_registry_base_colors() -> Vec<RegistryStyle> {
    vec![
        RegistryStyle {
            name: "slate".into(),
            label: "Slate".into(),
        },
        RegistryStyle {
            name: "gray".into(),
            label: "Gray".into(),
        },
        RegistryStyle {
            name: "zinc".into(),
            label: "Zinc".into(),
        },
        RegistryStyle {
            name: "neutral".into(),
            label: "Neutral".into(),
        },
        RegistryStyle {
            name: "stone".into(),
            label: "Stone".into(),
        },
    ]
}

pub async fn get_registry_base_color(
    base_color: &str,
) -> Result<RegistryBaseColor, Box<dyn Error>> {
    Ok(
        fetch_registry::<RegistryBaseColor>(vec![format!("colors/{}.json", base_color)])
            .await?
            .remove(0),
    )
}

pub fn resolve_tree(index: &Vec<RegistryItem>, names: &Vec<String>) -> Vec<RegistryItem> {
    let mut tree: Vec<RegistryItem> = vec![];

    for name in names {
        if let Some(item) = index.iter().find(|item| item.name == *name) {
            tree.push(item.clone());

            if let Some(registry_dependencies) = &item.registry_dependencies {
                tree.append(&mut resolve_tree(index, registry_dependencies));
            }
        }
    }

    tree.iter()
        .enumerate()
        .filter(|&(index, item)| {
            tree.iter()
                .position(|i| i.name == item.name)
                .is_some_and(|position| position == index)
        })
        .map(|(_, component)| component)
        .cloned()
        .collect()
}

pub async fn fetch_tree(
    style: &str,
    tree: Vec<RegistryItem>,
) -> Result<Vec<RegistryItemWithContent>, Box<dyn Error>> {
    let paths: Vec<String> = tree
        .iter()
        .map(|item| format!("styles/{}/{}.json", style, item.name))
        .collect();

    fetch_registry::<RegistryItemWithContent>(paths).await
}

pub fn get_item_target_path(
    _config: &Config,
    _item: &RegistryItemWithContent,
    r#override: Option<&Path>,
) -> PathBuf {
    if let Some(r#override) = r#override {
        return r#override.to_path_buf();
    }

    // if (item.r#type == RegistryItemType::Ui) {
    //     return config.resolved_paths.ui
    // }

    todo!("get_item_target_path")
}

async fn fetch_registry<T: DeserializeOwned>(paths: Vec<String>) -> Result<Vec<T>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut responses = vec![];

    for path in paths {
        let response = client
            .get(format!("{}/registry/{}", BASE_URL, path))
            .send()
            .await?;

        responses.push(response.json::<T>().await?);
    }

    Ok(responses)
}
