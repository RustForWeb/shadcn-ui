use std::error::Error;

use futures::{stream, StreamExt};
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::utils::get_config::Config;
use crate::utils::registry::schema::{RegistryItem, RegistryItemType};

const BASE_URL: &str = "https://ui.shadcn.com";

pub async fn get_registry_index() -> Vec<RegistryItem> {
    let results = fetch_registry::<Vec<RegistryItem>>(vec!["index.json".into()]).await;
    results.first().expect("TODO")
}

pub fn resolve_tree(index: &Vec<RegistryItem>, names: &Vec<String>) -> Vec<RegistryItem> {
    let mut tree: Vec<RegistryItem> = vec![];

    for name in names {
        if let Some(item) = index.iter().find(|item| item.name == *name) {
            tree.push(item.clone());

            if !item.registry_dependencies.is_empty() {
                tree.append(&mut resolve_tree(index, &item.registry_dependencies));
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

pub fn fetch_tree(style: String, tree: Vec<RegistryItem>) {
    let paths: Vec<String> = tree
        .iter()
        .map(|item| format!("styles/{}/{}.json", style, item.name))
        .collect();

    let result = fetch_registry(paths);
}

pub fn get_item_target_path(
    config: Config,
    item: RegistryItem,
    r#override: Option<String>,
) -> String {
    if let Some(r#override) = r#override {
        return r#override;
    }

    // if (item.r#type == RegistryItemType::Ui) {
    //     return config.resolved_paths.ui
    // }

    todo!("get_item_target_path")
}

async fn fetch_registry<T: DeserializeOwned + Send + 'static>(paths: Vec<String>) -> Vec<T> {
    let client = reqwest::Client::new();

    let responses = stream::iter(paths)
        .map(|path| {
            let client = client.clone();
            tokio::spawn(async move {
                let response = client
                    .get(format!("{}/registry/{}", BASE_URL, path))
                    .send()
                    .await?;
                response.bytes().await
            })
        })
        .buffer_unordered(10);

    vec![]
}
