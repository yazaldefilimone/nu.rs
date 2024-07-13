// export default async function install(packageNames: PackageName[], option: InstallOption = {}) {}

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
pub fn nu_install(package_names: Vec<String>, option: InstallOption) {}
