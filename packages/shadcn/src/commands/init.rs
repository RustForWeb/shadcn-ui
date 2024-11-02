use std::{env, path::PathBuf};

use anyhow::Result;
use clap::Args;

use crate::{
    preflights::preflight_init::pre_flight_init,
    utils::{errors::ErrorType, get_project_info::get_project_info, logger::LOGGER},
};

fn _default_cwd() -> PathBuf {
    env::current_dir().expect("Current directory should be accessible.")
}

#[derive(Args)]
pub struct InitOptions {
    #[arg(help = "the components to add or a url to the component.")]
    pub components: Vec<String>,

    #[arg(short, long, help = "skip confirmation prompt.")]
    pub yes: bool,

    #[arg(short, long, help = "use default configuration.")]
    pub defaults: bool,

    #[arg(short, long, help = "force overwrite of existing configuration.")]
    pub force: bool,

    #[arg(
        short,
        long,
        help = "the working directory. defaults to the current directory.",
        default_value = "."
    )]
    pub cwd: PathBuf,

    #[arg(short, long, help = "mute output.")]
    pub silent: bool,

    #[arg(long, help = "use the src directory when creating a new project.")]
    pub src_dir: bool,

    #[arg(skip)]
    pub skip_preflight: bool,
}

pub async fn init(options: InitOptions) -> Result<()> {
    let project_info = if !options.skip_preflight {
        let mut preflight = pre_flight_init(options).await?;
        if preflight
            .errors
            .remove(&ErrorType::MissingDirOrEmptyProject)
            .unwrap_or_default()
        {
            // TODO: create project
        }
        preflight.project_info
    } else {
        Some(get_project_info(&options.cwd).await?)
    };

    // TODO

    LOGGER.info("Success! Project initialization completed.\nYou may now add components.");
    LOGGER.r#break();

    Ok(())
}
