use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

use crate::resolver::ResolvedPackageInfo;
use std::{collections::HashMap, path::PathBuf, sync::Mutex};

#[derive(Debug, Default)]
struct LockFileCache {
  entries: Mutex<HashMap<String, ResolvedPackageInfo>>,
}

impl LockFileCache {
  async fn add_lock(&self, name: String, version: String, package_info: ResolvedPackageInfo) {
    let mut entries = self.entries.lock().unwrap();
    let name_version = format!("{}@{}", name, version);
    entries.insert(name_version, package_info);
  }

  async fn register_package_lock_cache(&self, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path).unwrap();
    let entries = &*self.entries.lock().unwrap();

    let json = serde_json::to_string(&entries).unwrap();

    file.write_all(json.as_bytes()).unwrap();

    Ok(())
  }

  async fn get_lock(&self, name: &str, version: &str) -> Option<ResolvedPackageInfo> {
    let mut entries = self.entries.lock().unwrap();
    let name_version = format!("{}@{}", name, version);
    if let Some(package_info) = entries.get(&name_version) {
      return Some(package_info.clone());
    }
    return None;
  }
}
