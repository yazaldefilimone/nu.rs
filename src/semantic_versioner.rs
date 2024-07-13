use rayon::prelude::*;
use semver::{Version, VersionReq};

pub fn semantic_versioner_is_valid(constraint: &str) -> bool {
  let constraint = VersionReq::parse(constraint);
  constraint.is_ok()
}

pub fn semantic_versioner_max_satisfying(
  versions: &[String],
  constraint: &str,
) -> Result<String, Box<dyn std::error::Error>> {
  let version_req = VersionReq::parse(constraint).ok().unwrap();
  let parsed_versions: Vec<Version> = versions
    .par_iter()
    .map(|version_str| Version::parse(version_str).unwrap())
    .collect();

  let version = parsed_versions
    .par_iter()
    .filter(|version| version_req.matches(version))
    .max()
    .unwrap();

  Ok(version.to_string())
}

pub fn semantic_versioner_is_valid_constraint(constraint: &str) -> bool {
  let constraint = VersionReq::parse(constraint);
  constraint.is_ok()
}

pub fn semantic_versioner_satisfies(version: &str, constraint: &str) -> bool {
  println!("version: {}, constraint: {}", version, constraint);

  let version = Version::parse(version);
  if version.is_err() {
    return false;
  }

  let version = version.unwrap();
  let constraint = VersionReq::parse(constraint);
  if constraint.is_err() {
    return false;
  }

  let constraint = constraint.unwrap();

  constraint.matches(&version)
}
