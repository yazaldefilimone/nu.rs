#![allow(unused)]

use crate::constants::get_package_json_path_buf;
use serde_json::Value;
use std::{collections::HashMap, fs, io};

pub type DependenciesMap = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct PackageDependency {
  pub dependencies: Option<DependenciesMap>,
  pub dev_dependencies: Option<DependenciesMap>,
}

pub fn parse_package_json() -> io::Result<PackageDependency> {
  let path_buf = get_package_json_path_buf();

  let package_json = fs::read_to_string(path_buf).expect("Failed to read package.json");
  let json_value: Value = serde_json::from_str(&package_json).expect("Failed to deserialize package.json");

  let dependencies_value = json_value.get("dependencies");
  let dev_dependencies_value = json_value.get("devDependencies");

  let dependencies = dependencies_value.and_then(|d| serde_json::from_value(d.clone()).ok());

  let dev_dependencies = dev_dependencies_value.and_then(|d| serde_json::from_value(d.clone()).ok());

  Ok(PackageDependency { dependencies, dev_dependencies })
}

pub fn write_package_json(new_dependencies: PackageDependency) -> io::Result<()> {
  let path = get_package_json_path_buf();
  let mut package_json_content = fs::read_to_string(&path).expect("Failed to read package.json");
  let mut json_value: Value = serde_json::from_str(&package_json_content).expect("Failed to deserialize package.json");

  if let Some(dependencies) = new_dependencies.dependencies {
    match json_value.get_mut("dependencies") {
      Some(existing_dependencies) => {
        let existing_dependencies = existing_dependencies.as_object_mut().unwrap();
        for (key, value) in dependencies {
          existing_dependencies.insert(key, Value::String(value));
        }
      }
      None => {
        json_value["dependencies"] = serde_json::json!(dependencies);
      }
    }
  }

  if let Some(dev_dependencies) = new_dependencies.dev_dependencies {
    match json_value.get_mut("devDependencies") {
      Some(existing_dev_dependencies) => {
        let existing_map = existing_dev_dependencies.as_object_mut().unwrap();
        for (key, value) in dev_dependencies {
          existing_map.insert(key, Value::String(value));
        }
      }
      None => {
        json_value["devDependencies"] = serde_json::json!(dev_dependencies);
      }
    }
  }

  package_json_content = serde_json::to_string_pretty(&json_value).expect("Failed to serialize package.json");

  fs::write(path, package_json_content).expect("Failed to write package.json");
  Ok(())
}
