#![allow(unused)]
use crate::constants::create_npm_registry_url;
use crate::logger::{install_package_log, not_found_resolve_package_log};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use tar::Archive;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmManifest {
  pub name: String,
  #[serde(rename = "dist-tags")]
  pub dist_tags: NpmManifestDistTags,
  pub versions: HashMap<String, NpmManifestVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmManifestDistTags {
  pub latest: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NpmManifestVersion {
  pub dist: NpmManifestVersionDist,
  pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NpmManifestVersionDist {
  pub tarball: String,
  pub shasum: String,
}

pub async fn fetch_package_manifest(package_name: &str) -> Result<NpmManifest, reqwest::Error> {
  let registry = create_npm_registry_url(package_name);
  let response = reqwest::get(&registry).await?;
  let manifest = response.json::<NpmManifest>().await?;
  Ok(manifest)
}

pub async fn download_and_extract_tarball(tar_url: &str, target: &str) -> Result<(), Box<dyn std::error::Error>> {
  let response = reqwest::get(tar_url).await?;
  let tarball = response.bytes().await?;

  // Create the target path directory if it doesn't exist
  fs::create_dir_all(target).await?;

  // Write the tarball to a temporary file
  let temp_tar_path = format!("{}/temp.tar.gz", target);
  let mut file = fs::File::create(&temp_tar_path).await?;
  file.write_all(&tarball).await?;
  file.flush().await?;

  // Extract the tarball
  let file = std::fs::File::open(&temp_tar_path).unwrap();
  let reader = GzDecoder::new(file);
  let mut archive = Archive::new(reader);
  archive.unpack(target)?;

  // Remove the temporary tarball file
  fs::remove_file(temp_tar_path).await?;

  Ok(())
}

pub async fn resolve_manifest(package_name: &str, version: &str) -> Result<NpmManifestVersion, Box<dyn Error>> {
  let manifest = fetch_package_manifest(package_name).await?;
  if let Some(package_version) = manifest.versions.get(version) {
    return Ok(package_version.clone());
  }
  not_found_resolve_package_log(package_name, version);
  Err(Box::new(std::io::Error::new(
    std::io::ErrorKind::NotFound,
    "Package not found",
  )))
}

pub async fn fetch_package_versions(package_name: &str) -> Result<Vec<String>, reqwest::Error> {
  let manifest = fetch_package_manifest(package_name).await?;
  let versions: Vec<String> = manifest.versions.keys().cloned().collect();
  Ok(versions)
}

pub async fn install_package(package_name: &str, version: &str, path_name: &str) -> Result<(), Box<dyn Error>> {
  let package_version = resolve_manifest(package_name, version).await?;
  download_and_extract_tarball(&package_version.dist.tarball, path_name).await?;
  install_package_log(package_name, version, path_name);
  Ok(())
}
