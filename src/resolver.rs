#![allow(unused)]

use std::collections::HashMap;
use std::sync::Arc;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::Mutex;

use crate::npm::fetch_package_manifest;
use crate::package::{DependenciesMap, PackageDependency};
use crate::semantic_versioner::{semantic_versioner_max_satisfying, semantic_versioner_satisfies};

pub type VersionConstraint = String;
pub type PackageName = String;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolvedPackageInfo {
  pub version: String,
  pub url: String,
  pub shasum: String,
  pub dependencies: HashMap<String, String>,
}

type ResolvedPackageResult = Result<ResolvedPackageInfo, Box<dyn std::error::Error>>;

pub async fn resolve_package_latest_version(package_name: &str) -> Result<String, Box<dyn std::error::Error>> {
  // const latestPackageInfo = await resolvePackage(packageName, '*')
  // if (!latestPackageInfo) throw new Error(`Package not found: ${packageName}`)
  // return latestPackageInfo.version
  //
  let latest_version = resolve_package(package_name, "*").await.unwrap();

  return Ok(latest_version.version);
}

pub async fn resolve_package(package_name: &str, version: &str) -> ResolvedPackageResult {
  let manifest = fetch_package_manifest(package_name).await?;

  let versions_keys: Vec<String> = manifest.versions.keys().cloned().collect();

  let max_version = semantic_versioner_max_satisfying(&versions_keys, &version)?;

  if !manifest.versions.contains_key(&max_version) {
    return Err(Box::new(std::io::Error::new(
      std::io::ErrorKind::NotFound,
      format!("Satisfied version not found: {}&{}", package_name, version),
    )));
  }

  let satisfied_version = manifest.versions.get(&max_version).unwrap().clone();

  return Ok(ResolvedPackageInfo {
    version: max_version,
    url: satisfied_version.dist.tarball.to_string(),
    shasum: satisfied_version.dist.shasum.to_string(),
    dependencies: satisfied_version.dependencies.unwrap_or(HashMap::new()),
  });
}

// todo: imprve, suport conflict resolution
pub async fn collect_dependencies_package_list(package_name: &str, version: &str, package_list: &mut DependenciesMap) {
  // resolve package info
  let package_info = resolve_package(package_name, version).await.unwrap();

  // add self to package list of installed packages
  package_list.insert(package_name.to_string(), package_info.version.clone());

  // add dependencies to package list of installed packages
  // todo: parallel (improve performance)
  for (name, version) in &package_info.dependencies {
    collect_dependencies_package_list(name, version, package_list);
  }
}

// parallel version
// pub async fn collect_dependencies_package_list(
//   package_name: &str,
//   version: &str,
//   package_list: Arc<Mutex<DependenciesMap>>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//   // Resolve package info
//   let package_info = resolve_package(package_name, version).await.unwrap();

//   // Add self to package list of installed packages
//   {
//     let mut package_list_guard = package_list.lock().await;
//     package_list_guard.insert(package_name.to_string(), package_info.version.to_string());
//   }

//   // Collect dependencies in parallel
//   let mut tasks = vec![];
//   for (name, version) in package_info.dependencies.iter() {
//     let name = name.clone();
//     let version = version.clone();
//     let package_list_clone = Arc::clone(&package_list);

//     let task = tokio::spawn(async move {
//       collect_dependencies_package_list(&name, &version, package_list_clone).await;
//     });

//     tasks.push(task);
//   }

//   // Await all tasks
//   for task in tasks {
//     task.await.unwrap();
//   }
// }
