use std::{
    collections::HashMap,
    env::current_dir,
    fs::{self, read_dir},
    path::{Path, PathBuf},
};

use indexmap::IndexMap;
use serde_json::Value;

use super::constants::{DEPENDENCIES_KEY, DEVELOPMENT_KEY, PROJECT_FILE, REQUIRED_KEY};
use super::lib::PackageManager;

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

pub fn get_project_files_for_all_projects(
    projects: &HashMap<String, String>,
) -> HashMap<String, PathBuf> {
    let mut projects_file_paths = HashMap::new();
    for (project_name, project_path) in projects {
        let project_file_path = Path::new(project_path).join(PROJECT_FILE);
        projects_file_paths.insert(project_name.clone(), project_file_path);
    }
    projects_file_paths
}

pub fn get_dependencies_from_file(
    file_path: &PathBuf,
) -> Option<(Vec<Value>, Vec<Value>, Value)> {
    let contents: IndexMap<String, Value> =
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
