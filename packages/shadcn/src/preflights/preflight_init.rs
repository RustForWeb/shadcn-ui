use std::collections::HashMap;

use anyhow::{bail, Result};
use tokio::fs;

use crate::{
    commands::init::InitOptions,
    utils::{
        errors::ErrorType,
        get_project_info::{get_project_info, ProjectInfo},
        highlighter::HIGHLIGHTER,
        logger::LOGGER,
        spinner::{spinner, SpinnerOptions},
    },
};

pub struct PreFlightInitResult {
    pub errors: HashMap<ErrorType, bool>,
    pub project_info: Option<ProjectInfo>,
}

pub async fn pre_flight_init(options: InitOptions) -> Result<PreFlightInitResult> {
    let mut errors: HashMap<ErrorType, bool> = HashMap::new();

    // Ensure target directory exists.
    // Check for empty project. We assume if no Cargo.toml exists, the project is empty.
    if !fs::try_exists(&options.cwd).await?
        || !fs::try_exists(options.cwd.join("Cargo.toml")).await?
    {
        errors.insert(ErrorType::MissingDirOrEmptyProject, true);

        return Ok(PreFlightInitResult {
            errors,
            project_info: None,
        });
    }

    let mut project_spinner = spinner(
        "Preflight checks.",
        SpinnerOptions {
            silent: options.silent,
        },
    );

    if fs::try_exists(options.cwd.join("components.toml")).await? && !options.force {
        project_spinner.fail();

        LOGGER.r#break();
        LOGGER.error(&format!(
            "A {} file already exists at {}.\nTo start over, remove the {} file and run {} again.",
            HIGHLIGHTER.info("components.toml"),
            HIGHLIGHTER.info(&options.cwd.to_string_lossy()),
            HIGHLIGHTER.info("components.toml"),
            HIGHLIGHTER.info("init"),
        ));
        LOGGER.r#break();

        bail!("");
    }

    project_spinner.succeed();

    // TODO

    let project_info = get_project_info(options.cwd).await?;

    Ok(PreFlightInitResult {
        errors,
        project_info: Some(project_info),
    })
}
