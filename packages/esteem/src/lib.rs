mod add;
mod init;
mod install_isolated;
mod remove;
use crate::init::Init;
use add::Add;
use core::fmt;
use install_isolated::InstallIsolated;
use remove::Remove;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env::current_dir,
    error::Error,
    fs::{self, read_dir},
    path::{Path, PathBuf},
    process,
};
#[macro_use]
extern crate log;

pub const WORKSPACE_FILE: &str = "workspace.json";
const PACKAGE_JSON_BACKUP_FILE: &str = "package.backup.json";
const PACKAGE_JSON_FILE: &str = "package.json";
const PROJECT_FILE: &str = "project.json";
const DEPENDENCIES_KEY: &str = "dependencies";
const DEVELOPMENT_DEPENDENCIES_KEY: &str = "devDependencies";
const REQUIRED_KEY: &str = "required";
const DEVELOPMENT_KEY: &str = "development";

#[derive(Debug)]
pub struct LibraryError;

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Library error")
    }
}

impl Error for LibraryError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub projects: HashMap<String, String>,
}

impl Workspace {
    pub fn new() -> Result<Self, LibraryError> {
        let workspace_file = fs::read_to_string(WORKSPACE_FILE);
        match workspace_file {
            Ok(data) => Ok(serde_json::from_str(&data).unwrap()),
            Err(_) => {
                trace!("Unable to find file: {:?}", WORKSPACE_FILE);
                Err(LibraryError)
            }
        }
    }
}

pub(crate) trait Command {
    fn execute(&self);
}

#[derive(Debug)]
pub(crate) enum PackageManager {
    Yarn,
    Pnpm,
    Npm,
}

pub(crate) fn get_npm_package_manager() -> Option<PackageManager> {
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

pub fn perform_add(project_path: PathBuf, is_development: bool, to_add: Vec<String>) {
    let npm_package_manager = get_npm_package_manager().unwrap_or_else(|| {
        error!("A valid lockfile was not found for this project.");
        process::exit(1);
    });
    let a = Add::new(project_path, is_development, to_add, npm_package_manager);
    a.execute();
}

pub fn perform_init(projects_file_paths: HashMap<String, PathBuf>) {
    let a = Init::new(projects_file_paths);
    a.execute();
}

pub fn perform_install_isolated(project_path: PathBuf) {
    let a = InstallIsolated::new(project_path);
    a.execute();
}

pub fn perform_remove(
    project_path: PathBuf,
    to_remove: Vec<String>,
    all_projects: HashMap<String, PathBuf>,
) {
    let npm_package_manager = get_npm_package_manager().unwrap_or_else(|| {
        error!("A valid lockfile was not found for this project.");
        process::exit(1);
    });
    let a = Remove::new(project_path, to_remove, all_projects, npm_package_manager);
    a.execute();
}
