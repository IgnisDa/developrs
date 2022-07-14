use core::fmt;
use std::{error::Error, fs::write, path::PathBuf};
mod constants;
mod dependencies;
mod managers;
mod project;
mod utils;
mod workspace;
use serde::Serialize;
mod init;
mod install_isolated;
mod remove;
pub use constants::{WORKSPACE_FILE, WORKSPACE_IDENTIFIER};
pub use dependencies::EsteemDependencies;
use std::{collections::BTreeMap, process};
use utils::{get_npm_package_manager, get_npm_package_manager_new};
pub use workspace::EsteemWorkspace;

#[macro_use]
extern crate log;

pub fn perform_add(
    project_name: String,
    to_add: Vec<String>,
    is_development: bool,
    skip_package_manager: bool,
) {
    let mut workspace = EsteemWorkspace::from_current_directory().unwrap();
    let project = workspace.get_project(project_name).unwrap();
    to_add.iter().for_each(|dependency| {
        if is_development {
            project.add_development_dependency(dependency.to_string())
        } else {
            project.add_required_dependency(dependency.to_string())
        }
    });
    project.write_dependencies();
    if !skip_package_manager {
        let mut manager = get_npm_package_manager_new().unwrap();
        manager.add_dependencies(to_add);
        manager.execute();
    }
}

pub fn perform_init(projects_file_paths: BTreeMap<String, PathBuf>) {
    let a = init::Init::new(projects_file_paths);
    a.execute();
}

pub fn perform_install_isolated(project_path: Vec<PathBuf>) {
    let a = install_isolated::InstallIsolated::new(project_path);
    a.execute();
}

pub fn perform_remove(
    project_path: PathBuf,
    to_remove: Vec<String>,
    all_projects: BTreeMap<String, PathBuf>,
) {
    let npm_package_manager = get_npm_package_manager().unwrap_or_else(|| {
        error!("A valid lockfile was not found for this project.");
        process::exit(1);
    });
    let a =
        remove::Remove::new(project_path, to_remove, all_projects, npm_package_manager);
    a.execute();
}

pub fn perform_workspace_add(
    to_add: Vec<String>,
    is_development: bool,
    skip_package_manager: bool,
) {
    let mut workspace = EsteemWorkspace::from_current_directory().unwrap();
    to_add.iter().for_each(|dependency| {
        if is_development {
            workspace.add_development_dependency(dependency.to_string())
        } else {
            workspace.add_required_dependency(dependency.to_string())
        }
    });
    workspace.write_dependencies();
    if !skip_package_manager {
        let mut manager = get_npm_package_manager_new().unwrap();
        manager.add_dependencies(to_add);
        manager.execute();
    }
}

#[derive(Debug)]
pub struct LibraryError;

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Library error")
    }
}

impl Error for LibraryError {}

pub trait Command {
    fn execute(&self);
}

#[derive(Debug)]
pub enum PackageManager {
    Yarn,
    Pnpm,
    Npm,
}

/// Used to add a required dependency to a project or workspace
pub trait AddEsteemRequiredDependency {
    fn add_required_dependency(&mut self, dependency: String);
}

/// Used to add a development dependency to a project or workspace
pub trait AddEsteemDevelopmentDependency {
    fn add_development_dependency(&mut self, dependency: String);
}

/// Used to write dependencies to a file
pub trait WriteDependencies
where
    Self: Serialize,
{
    fn get_path(&self) -> PathBuf;

    fn write_dependencies(&self) {
        info!("Writing new dependencies to {:?}", self.get_path());
        let to_write = serde_json::to_string_pretty(self).unwrap();
        write(self.get_path(), to_write).unwrap();
    }
}
