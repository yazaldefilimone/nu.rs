#![allow(unused)]

use clap::{arg, Arg, ArgAction, Command};

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
            .required(true)
            .num_args(1..),
        )
        .arg(arg!(-D --dev "Save as a development dependency").required(false)),
    );
  return nu_command;
}
