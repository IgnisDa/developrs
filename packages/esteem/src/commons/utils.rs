use std::{
    collections::BTreeMap,
    env::current_dir,
    fs::{self, read_dir},
    path::PathBuf,
};

use serde_json::Value;

use crate::commons::{
    lib::PackageManager,
    managers::{NpmManager, PnpmManager, YarnManager},
};

use super::{
    constants::{DEPENDENCIES_KEY, DEVELOPMENT_KEY, REQUIRED_KEY},
    managers::AddNpmDependenciesAndExecuteNpmPackageManager,
};

pub fn get_npm_package_manager() -> Option<PackageManager> {
    let dir = read_dir(current_dir().unwrap()).unwrap();
    for file in dir {
        match file.unwrap().file_name().to_os_string().to_str().unwrap() {
            "yarn.lock" => return Some(PackageManager::Yarn),
            "pnpm-lock.yaml" => return Some(PackageManager::Pnpm),
            "package-lock.json" => return Some(PackageManager::Npm),
            _ => continue,
        }
    }
    warn!("No package manager lockfile found, early termination imminent.");
    None
}

pub fn get_npm_package_manager_new(
) -> Option<Box<dyn AddNpmDependenciesAndExecuteNpmPackageManager>> {
    let dir = read_dir(current_dir().unwrap()).unwrap();
    for file in dir {
        match file.unwrap().file_name().to_os_string().to_str().unwrap() {
            "yarn.lock" => return Some(Box::new(YarnManager::new())),
            "pnpm-lock.yaml" => return Some(Box::new(PnpmManager::new())),
            "package-lock.json" => return Some(Box::new(NpmManager::new())),
            _ => continue,
        }
    }
    warn!("No package manager lockfile found, early termination imminent.");
    None
}
pub fn get_dependencies_from_file(
    file_path: &PathBuf,
) -> Option<(Vec<Value>, Vec<Value>, Value)> {
    let contents: BTreeMap<String, Value> =
        serde_json::from_str(&fs::read_to_string(file_path).unwrap()).unwrap();
    let dependencies = contents.get(DEPENDENCIES_KEY).cloned();
    if let Some(project_dependencies) = dependencies {
        let to_install_required_deps = project_dependencies[REQUIRED_KEY]
            .as_array()
            .unwrap()
            .clone();
        let to_install_development_deps = project_dependencies[DEVELOPMENT_KEY]
            .as_array()
            .unwrap()
            .clone();
        Some((
            to_install_required_deps,
            to_install_development_deps,
            project_dependencies,
        ))
    } else {
        None
    }
}
