use flate2::read::GzDecoder;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tar::Archive;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::constants::create_npm_registry_url;
use crate::logger::{install_package_log, not_found_resolve_package_log};

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmManifest {
  name: String,
  #[serde(rename = "dist-tags")]
  dist_tags: NpmManifestDistTags,
  versions: HashMap<String, NpmManifestVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmManifestDistTags {
  latest: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmManifestVersion {
  dist: NpmManifestVersionDist,
  dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmManifestVersionDist {
  tarball: String,
  shasum: String,
}

pub async fn fetch_package_manifest(package_name: &str) -> Result<NpmManifest, reqwest::Error> {
  let registry = create_npm_registry_url(package_name);
  let response = reqwest::get(&registry).await?;
  let manifest = response.json::<NpmManifest>().await?;
  Ok(manifest)
}

pub async fn download_and_extract_tarball(tar_url: &str, target: &str) -> Result<(), Box<dyn Error>> {
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
  let file = fs::File::open(&temp_tar_path).await?;
  let reader = GzDecoder::new(file.compat());
  let mut archive = Archive::new(reader);
  archive.unpack(target)?;

  // Remove the temporary tarball file
  fs::remove_file(temp_tar_path).await?;

  Ok(())
}

pub async fn resolve_manifest(package_name: &str, version: &str) -> Result<NpmManifestVersion, Box<dyn Error>> {
  let manifest = fetch_package_manifest(package_name).await?;
  if let Some(package_version) = manifest.versions.get(version) {
    Ok(package_version.clone())
  } else {
    Err(not_found_resolve_package_log(version, package_name))
  }
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
