use std::{error::Error, path::PathBuf};

use crate::utils::{
    get_config::get_config,
    registry::{get_registry_index, resolve_tree},
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

pub fn add(options: AddOptions) -> Result<(), Box<dyn Error>> {
    println!("{:?}", options);

    if !options.cwd.exists() {
        return Err(format!(
            "Path {} does not exist. Please try again.",
            options.cwd.display()
        )
        .into());
    }

    if let Some(config) = get_config(&options.cwd) {
        let registry_index = get_registry_index();

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

        Ok(())
    } else {
        Err("Configuration is missing. Please run `init` to create a components.toml file.".into())
    }
}
