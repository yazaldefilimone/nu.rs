#![allow(unused)]

use std::{collections::HashMap, error::Error};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
  npm::install_package,
  package::{parse_package_json, DependenciesMap},
  resolver::{collect_dependencies_package_list, resolve_package, resolve_package_latest_version},
};
#[derive(Debug, Clone, Copy)]
pub enum InstallOption {
  Global,
  Save,
  SaveDev,
  SaveOptional,
  SavePeer,
  SaveExact,
  SaveBundle,
}

pub type ConflictedPackageInfo = HashMap<String, Vec<String>>;

pub async fn command_install(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
  let packages: Vec<_> = matches
    .get_many::<String>("package")
    .expect("expected package name")
    .map(|package| package.as_str())
    .collect();

  let as_dev_deps = matches.get_flag("dev");

  let mut package_josn = parse_package_json().unwrap();

  // read package.lock.json
  //
  // todo: parallel
  for package_name in packages {
    // node@18.16.19 -> node -> 18.16.19
    //
    let has_constraint = package_name.contains('@');

    let name = if has_constraint {
      package_name.split('@').nth(0).unwrap().to_string()
    } else {
      package_name.to_string()
    };

    let constraint = if has_constraint {
      package_name.split('@').nth(1).unwrap().to_string()
    } else {
      format!("^{}", resolve_package_latest_version(&name).await.unwrap())
    };
    if !as_dev_deps {
      if let Some(dependency) = package_josn.dependencies.as_mut() {
        dependency.insert(name.clone(), constraint.clone());
      }
      let mut dependencies = HashMap::new();
      dependencies.insert(name.clone(), constraint.clone());
      package_josn.dependencies = Some(dependencies);
    } else {
      if let Some(dependency) = package_josn.dev_dependencies.as_mut() {
        dependency.insert(name.clone(), constraint.clone());
      }
      let mut dependencies = HashMap::new();
      dependencies.insert(name.clone(), constraint.clone());
      package_josn.dev_dependencies = Some(dependencies);
    }
  }

  let mut dependencies_map: DependenciesMap = HashMap::new();

  if !as_dev_deps {
    dependencies_map.extend(package_josn.dependencies.unwrap_or(HashMap::new()));
  }

  if as_dev_deps {
    dependencies_map.extend(package_josn.dev_dependencies.unwrap_or(HashMap::new()));
  }

  // let top_level_package_list: DependenciesMap = HashMap::new();

  // let conflicted_packageList: ConflictedPackageInfo = HashMap::new();

  let mut _dependencies_map = dependencies_map.clone();
  for (name, version) in dependencies_map.into_iter() {
    collect_dependencies_package_list(name.as_str(), version.as_str(), &mut _dependencies_map).await;
  }

  for (name, version) in _dependencies_map.into_iter() {
    install_package(name.as_str(), version.as_str(), "node_modules")
      .await
      .unwrap();
  }

  Ok(())
}
