use clap::{command, Arg, ArgAction, Command};

fn main() {
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
                        .help("The working directory, defaults to the current directory"),
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
                        .help("The path to add the component to"),
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
        Some(("add", sub_matches)) => println!(
            "'myapp add' was used, name is: {:?}",
            sub_matches
                .get_many::<String>("component")
                .unwrap_or_default()
                .map(|v| v.as_str())
                .collect::<Vec<_>>()
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
