use std::collections::HashMap;

use anyhow::{Result, bail};
use tokio::fs;

use crate::{
    commands::init::InitOptions,
    utils::{
        errors::ErrorType,
        get_project_info::{ProjectInfo, get_project_info},
        highlighter::HIGHLIGHTER,
        logger::LOGGER,
        spinner::{SpinnerOptions, spinner},
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

    project_spinner.succeed(None);

    // let framework_spinner = spinner("Verifying framework.", SpinnerOptions { silent: true });
    let project_info = get_project_info(&options.cwd).await?;
    // TODO
    // if project_info.framework.name == "manual" {}
    // framework_spinner.succeed(Some(format!("Verifying framework. Found {}.", HIGHLIGHTER.info(project_info.framework.label))));

    let mut tailwind_spinner = spinner("Validating Tailwind CSS.", SpinnerOptions { silent: true });
    if project_info.tailwind_config_file.is_none() || project_info.tailwind_css_file.is_none() {
        errors.insert(ErrorType::TailwindNotConfigured, true);
        tailwind_spinner.fail();
    } else {
        tailwind_spinner.succeed(None);
    }

    if !errors.is_empty() {
        if errors
            .get(&ErrorType::TailwindNotConfigured)
            .copied()
            .unwrap_or_default()
        {
            LOGGER.r#break();
            LOGGER.error(&format!(
                "No Tailwind CSS configuration found at {}.",
                HIGHLIGHTER.info(&options.cwd.to_string_lossy())
            ));
            LOGGER.error("It is likely you do not have Tailwind CSS installed or have an invalid configuration.");
            LOGGER.error("Install Tailwind CSS then try again.");

            // TODO: framework link
        }

        LOGGER.r#break();
        bail!("");
    }

    Ok(PreFlightInitResult {
        errors,
        project_info: Some(project_info),
    })
}
