use std::{error::Error, path::PathBuf};

use log::info;
use tokio::fs;

use crate::utils::{
    get_config::get_config,
    registry::{
        fetch_tree, get_item_target_path, get_registry_base_color, get_registry_index, resolve_tree,
    },
    transformers::{transform, TransformOptions},
};

#[derive(Debug)]
pub struct AddOptions {
    pub components: Vec<String>,
    pub yes: bool,
    pub overwrite: bool,
    pub cwd: PathBuf,
    pub all: bool,
    pub path: Option<PathBuf>,
}

pub async fn add(options: AddOptions) -> Result<(), Box<dyn Error>> {
    if !options.cwd.exists() {
        return Err(format!(
            "Path {} does not exist. Please try again.",
            options.cwd.display()
        )
        .into());
    }

    if let Some(config) = get_config(&options.cwd) {
        let registry_index = get_registry_index().await?;

        let selected_components = match options.all {
            true => registry_index
                .iter()
                .map(|item| item.name.clone())
                .collect(),
            false => options.components,
        };

        // TODO: prompt

        if selected_components.is_empty() {
            return Err("No components selected.".into());
        }

        let tree = resolve_tree(&registry_index, &selected_components);
        let payload = fetch_tree(&config.style, tree).await?;
        let base_color = get_registry_base_color(&config.tailwind.base_color).await?;

        if payload.is_empty() {
            return Err("Selected components not found.".into());
        }

        // TODO: yes prompt
        // TODO: spinner

        info!("Installing components...");
        for item in payload {
            info!("Installing {}...", item.name);

            let target_dir = get_item_target_path(&config, &item, options.path.as_deref());

            if !target_dir.exists() {
                fs::create_dir_all(&target_dir).await?;
            }

            let existing_component = item
                .files
                .iter()
                .any(|file| target_dir.join(file.name.clone()).exists());

            if existing_component && !options.overwrite {
                if selected_components.contains(&item.name) {
                    // TODO: prompt
                } else {
                    continue;
                }
            }

            for file in item.files {
                let file_path = target_dir.join(file.name.clone());

                let content = transform(TransformOptions {
                    filename: file.name,
                    raw: file.content,
                    config: &config,
                    base_color: Some(&base_color),
                })
                .await?;

                fs::write(file_path, content).await?;
            }
        }

        // TODO: dependencies

        info!("Done.");

        Ok(())
    } else {
        Err("Configuration is missing. Please run `init` to create a components.toml file.".into())
    }
}
