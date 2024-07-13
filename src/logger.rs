#![allow(unused)]

pub fn resolve_manifest_log(package_name: &str, version: &str) {
  println!("[Resolved manifest...] {}@{}", package_name, version);
}

pub fn fetch_package_manifest_log(package_name: &str) {
  println!("[Fetched manifest...] {}", package_name);
}

pub fn fetch_package_versions_log(package_name: &str) {
  println!("[Fetched...] {}", package_name);
}

pub fn install_package_log(package_name: &str, version: &str, path_name: &str) {
  println!("[Installed...] {}@{} > {}", package_name, version, path_name);
}

pub fn resolve_package_log(package_name: &str, version: &str) {
  println!("[Resolved...] {}@{}", package_name, version);
}

pub fn not_found_resolve_package_log(package_name: &str, version: &str) {
  println!("[Not found...] version {} for package {}", version, package_name);
}

pub fn removel_package_log(package_name: &str, version: &str, path_name: &str) {
  println!("[Removed...] {}@{} > {}", package_name, version, path_name);
}
