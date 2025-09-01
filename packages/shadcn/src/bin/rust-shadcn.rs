use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use shadcn::commands::init::{InitOptions, init};
use shadcn::commands::generate::{GenerateArgs, generate};

#[derive(Parser)]
#[command(version, propagate_version = true)]
#[command(about = "add components and dependencies to your project")]
// #[command(subcommand_required = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "add a component to your project")]
    Add(AddArgs),
    #[command(about = "check for updates against the registry")]
    Diff(DiffArgs),
    #[command(about = "generate a new component scaffold")]
    Generate(GenerateArgs),
    #[command(about = "initialize your project and install dependencies")]
    Init(InitOptions),
}

#[derive(Args)]
struct AddArgs {}

#[derive(Args)]
struct DiffArgs {}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(_args) => Ok(()),
        Commands::Diff(_args) => Ok(()),
        Commands::Generate(args) => generate(args).await,
        Commands::Init(args) => init(args).await,
    }
}
