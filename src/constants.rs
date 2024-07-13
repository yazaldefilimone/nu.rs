use std::{env, path::PathBuf};

pub const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org";

pub fn create_npm_registry_url(package_name: &str) -> String {
  format!("{}{}", NPM_REGISTRY_URL, package_name)
}

pub fn get_package_json_path_buf() -> PathBuf {
  let current_dir = env::current_dir().expect("Failed to get current directory");
  current_dir.join("package.json")
}

pub fn get_npm_package_lock_path_buf() -> PathBuf {
  let current_dir = env::current_dir().expect("Failed to get current directory");
  current_dir.join("package-lock.json")
}

// pnpm

pub fn get_pnpm_package_lock_path_buf() -> PathBuf {
  let current_dir = env::current_dir().expect("Failed to get current directory");
  current_dir.join("pnpm-lock.yaml")
}
