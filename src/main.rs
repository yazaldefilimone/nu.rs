mod cli;
mod constants;
mod install;
mod lock;
mod logger;
mod npm;
mod package;
mod resolver;

fn nu_install(matches: &clap::ArgMatches) {
  let package: Vec<_> = matches
    .get_many::<String>("package")
    .expect("expected package name")
    .map(|package| package.as_str())
    .collect();

  let as_dev_deps = matches.get_flag("dev");
  let with_types = matches.get_flag("types");
  println!("Installing {}...", package.join(", "));

  if as_dev_deps {
    println!("Installing {} as dev dependencies...", package.join(", "));
  }
  if with_types {
    let packages_with_types: Vec<_> = package.iter().map(|package| format!("@types/{}", package)).collect();
    println!("Installing {} as dev dependencies...", packages_with_types.join(", "));
  }
}

fn nu_remove(matches: &clap::ArgMatches) {
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
}

fn nu_update(matches: &clap::ArgMatches) {
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
}

fn nu_upgrade(matches: &clap::ArgMatches) {
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
}

fn main() {
  let matches_command = cli::nu_cli().get_matches();
  match matches_command.subcommand() {
    Some(("install", sub_matches)) => {
      nu_install(&sub_matches);
      let packages: Vec<_> = sub_matches
        .get_many::<String>("package")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
      let values = packages.join(", ");
      if sub_matches.get_flag("dev") {
        println!("Installing {} as dev dependencies...", values);
      } else {
        println!("Installing {}...", values);
      }
    }
    Some(("remove", sub_matches)) => {
      let packages: Vec<_> = sub_matches
        .get_many::<String>("package")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
      let values = packages.join(", ");
      println!("Removing {}...", values);
    }
    Some(("update", sub_matches)) => {
      let packages: Vec<_> = sub_matches
        .get_many::<String>("package")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
      let values = packages.join(", ");
      println!("Updating {}...", values);
    }
    Some(("upgrade", sub_matches)) => {
      let packages: Vec<_> = sub_matches
        .get_many::<String>("package")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
      let values = packages.join(", ");
      println!("Upgrading {}...", values);
    }
    Some(("list", _sub_matches)) => {
      println!("Listing installed packages...");
    }
    Some(("search", sub_matches)) => {
      let packages: Vec<_> = sub_matches
        .get_many::<String>("package")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
      let values = packages.join(", ");
      println!("Searching for {}...", values);
    }
    _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
  }
}
