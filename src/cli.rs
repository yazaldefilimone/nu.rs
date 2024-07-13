use clap::{Arg, ArgAction, Command};

pub fn nu_cli() -> Command {
  let nu_command = Command::new("nu")
    .about("Nu Package Manager")
    .version("0.1.0")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("install")
        .about("Install packages")
        .arg(
          Arg::new("package")
            .help("Packages to install")
            .action(ArgAction::Set)
            .num_args(1..),
        )
        .arg(
          Arg::new("dev")
            .help("Save as a development dependency")
            .short_alias('D')
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("types")
            .help("Include @types packages as dev dependencies")
            .short_alias('T')
            .action(ArgAction::SetTrue),
        ),
    )
    .subcommand(
      Command::new("remove")
        .about("Remove packages")
        .arg(
          Arg::new("package")
            .help("Packages to remove")
            .action(ArgAction::Set)
            .num_args(1..),
        )
        .arg(
          Arg::new("dev")
            .help("Remove as a development dependency")
            .short_alias('D')
            .action(ArgAction::SetTrue),
        ),
    )
    .subcommand(
      Command::new("update")
        .about("Update packages")
        .arg(
          Arg::new("package")
            .help("Packages to update")
            .action(ArgAction::Set)
            .num_args(1..),
        )
        .arg(
          Arg::new("dev")
            .help("Update as a development dependency")
            .short_alias('D')
            .action(ArgAction::SetTrue),
        ),
    )
    .subcommand(
      Command::new("upgrade")
        .about("Upgrade packages")
        .arg(
          Arg::new("package")
            .help("Packages to upgrade")
            .action(ArgAction::Set)
            .num_args(1..),
        )
        .arg(
          Arg::new("dev")
            .help("Upgrade as a development dependency")
            .short_alias('D')
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("types")
            .help("Include @types packages as dev dependencies")
            .short_alias('T')
            .action(ArgAction::SetTrue),
        ),
    )
    .subcommand(
      Command::new("list")
        .long_flag("list")
        .about("List installed packages")
        .arg(
          Arg::new("package")
            .help("Packages to upgrade")
            .action(ArgAction::Set)
            .num_args(1..),
        ),
    )
    .subcommand(
      Command::new("search")
        .long_flag("search")
        .about("Search for package on npm registry")
        .arg(
          Arg::new("package")
            .help("Package to search for")
            .action(ArgAction::Set)
            .num_args(1..),
        ),
    );
  return nu_command;
}
