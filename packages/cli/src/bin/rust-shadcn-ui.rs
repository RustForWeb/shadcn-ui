use std::{env, error::Error, path::PathBuf};

use clap::{command, value_parser, Arg, ArgAction, Command};
use shadcn_ui_cli::commands::{add, AddOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a component to your project")
                .arg(
                    Arg::new("component")
                        .action(ArgAction::Append)
                        .help("The components to add"),
                )
                .arg(
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .action(ArgAction::SetTrue)
                        .help("Add all available components"),
                )
                .arg(
                    Arg::new("cwd")
                        .short('c')
                        .long("cwd")
                        .help("The working directory, defaults to the current directory")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new("overwrite")
                        .short('o')
                        .long("overwrite")
                        .action(ArgAction::SetTrue)
                        .help("Overwrite existing files"),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .help("The path to add the component to")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .long("yes")
                        .help("Skip confirmation prompt")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("diff")
                .about("Check for updates against the registry")
                .arg(Arg::new("component").help("The component name"))
                .arg(
                    Arg::new("cwd")
                        .short('c')
                        .long("cwd")
                        .help("The working directory, defaults to the current directory"),
                )
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .long("yes")
                        .help("Skip confirmation prompt")
                        .action(ArgAction::SetTrue),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => add(AddOptions {
            components: sub_matches
                .get_many::<String>("component")
                .unwrap_or_default()
                .cloned()
                .collect::<Vec<_>>(),
            all: sub_matches.get_flag("all"),
            cwd: sub_matches.get_one::<PathBuf>("cwd").cloned().unwrap_or(env::current_dir().expect("Current directory does not exist or there are insufficient permissions to access it.")),
            overwrite: sub_matches.get_flag("overwrite"),
            path: sub_matches.get_one::<PathBuf>("path").cloned(),
            yes: sub_matches.get_flag("yes"),
        }).await,
        Some(("diff", _sub_matches)) => Err("Command `diff` is not yet implemented".into()),
        Some(("init", _sub_matches)) => Err("Command `init` is not yet implemented.".into()),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
