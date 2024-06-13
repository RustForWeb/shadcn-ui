use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawConfig {
    style: String,
    tailwind: TailwindConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TailwindConfig {
    config: String,
    css: String,
    base_color: String,
    css_variables: bool,
    prefix: String,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub style: String,
    pub tailwind: TailwindConfig,
    pub resolved_paths: ResolvedPaths,
}

#[derive(Clone, Debug)]
pub struct ResolvedPaths {
    pub tailwind_config: PathBuf,
    pub tailwind_css: PathBuf,
}

pub fn get_config(cwd: &Path) -> Option<Config> {
    get_raw_config(cwd).map(|config| resolve_config_paths(cwd, config))
}

pub fn resolve_config_paths(cwd: &Path, config: RawConfig) -> Config {
    Config {
        style: config.style,
        tailwind: config.tailwind.clone(),
        resolved_paths: ResolvedPaths {
            tailwind_config: cwd.join(config.tailwind.config),
            tailwind_css: cwd.join(config.tailwind.css),
        },
    }
}

pub fn get_raw_config(cwd: &Path) -> Option<RawConfig> {
    // TODO: use search algorithm
    let config_path = cwd.join("components.toml");

    if !config_path.exists() {
        return None;
    }

    fs::read_to_string(config_path)
        .ok()
        .and_then(|config_content| toml::from_str(&config_content).ok())
}
