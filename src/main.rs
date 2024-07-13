#![allow(unused)]

mod cache;
mod cli;
mod constants;
mod install;
mod lock;
mod logger;
mod npm;
mod package;
mod resolver;
mod semantic_versioner;
use install::command_install;
use std::error::Error;

fn nu_remove(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
  let package: Vec<_> = matches
    .get_many::<String>("package")
    .expect("expected package name")
    .map(|package| package.as_str())
    .collect();

  let as_dev_deps = matches.get_flag("dev");

  let with_types = matches.get_flag("types");
  println!("Removing {}...", package.join(", "));

  if as_dev_deps {
    println!("Removing {} as dev dependencies...", package.join(", "));
  }
  if with_types {
    let packages_with_types: Vec<_> = package.iter().map(|package| format!("@types/{}", package)).collect();
    println!("Removing {} as dev dependencies...", packages_with_types.join(", "));
  }
  Ok(())
}

fn nu_update(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
  let package: Vec<_> = matches
    .get_many::<String>("package")
    .expect("expected package name")
    .map(|package| package.as_str())
    .collect();

  let as_dev_deps = matches.get_flag("dev");

  let with_types = matches.get_flag("types");
  println!("Updating {}...", package.join(", "));

  if as_dev_deps {
    println!("Updating {} as dev dependencies...", package.join(", "));
  }
  if with_types {
    let packages_with_types: Vec<_> = package.iter().map(|package| format!("@types/{}", package)).collect();
    println!("Updating {} as dev dependencies...", packages_with_types.join(", "));
  }
  Ok(())
}

fn nu_upgrade(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
  let package: Vec<_> = matches
    .get_many::<String>("package")
    .expect("expected package name")
    .map(|package| package.as_str())
    .collect();

  let as_dev_deps = matches.get_flag("dev");

  let with_types = matches.get_flag("types");
  println!("Upgrading {}...", package.join(", "));

  if as_dev_deps {
    println!("Upgrading {} as dev dependencies...", package.join(", "));
  }
  if with_types {
    let packages_with_types: Vec<_> = package.iter().map(|package| format!("@types/{}", package)).collect();
    println!("Upgrading {} as dev dependencies...", packages_with_types.join(", "));
  }
  Ok(())
}

#[tokio::main]
async fn main() {
  let matches_command = cli::nu_cli().get_matches();
  let result = match matches_command.subcommand() {
    Some(("install", sub_matches)) => command_install(&sub_matches).await,
    _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
  };

  if let Err(e) = result {
    eprintln!("Error: {}", e);
  }
}
