use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct InitOptions {
    #[arg(help = "the components to add or a url to the component.")]
    components: Vec<String>,

    #[arg(short, long, help = "skip confirmation prompt.")]
    yes: bool,

    #[arg(short, long, help = "use default configuration.")]
    defaults: bool,

    #[arg(short, long, help = "force overwrite of existing configuration.")]
    force: bool,

    #[arg(
        short,
        long,
        help = "the working directory. defaults to the current directory."
    )]
    cwd: PathBuf,

    #[arg(short, long, help = "mute output.")]
    silent: bool,

    #[arg(long, help = "use the src directory when creating a new project.")]
    src_dir: bool,
}

pub async fn init(options: InitOptions) -> Result<()> {
    Ok(())
}
