mod add;
mod commons;
mod init;
mod install_isolated;
mod remove;
use std::{collections::BTreeMap, path::PathBuf, process};

pub use commons::{
    constants::{WORKSPACE_FILE, WORKSPACE_IDENTIFIER},
    dependencies::EsteemDependencies,
    lib::{AddEsteemRequiredDependency, WriteDependencies},
    workspace::EsteemWorkspace,
};
use commons::{
    lib::{AddEsteemDevelopmentDependency, Command},
    utils::{get_npm_package_manager, get_npm_package_manager_new},
};

#[macro_use]
extern crate log;

pub fn perform_add(project_name: String, is_development: bool, to_add: Vec<String>) {
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
    let mut manager = get_npm_package_manager_new().unwrap();
    manager.add_dependencies(to_add);
    manager.execute();
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

pub fn perform_workspace_add(is_development: bool, to_add: Vec<String>) {
    let mut workspace = EsteemWorkspace::from_current_directory().unwrap();
    to_add.iter().for_each(|dependency| {
        if is_development {
            workspace.add_development_dependency(dependency.to_string())
        } else {
            workspace.add_required_dependency(dependency.to_string())
        }
    });
    workspace.write_dependencies();
    let mut manager = get_npm_package_manager_new().unwrap();
    manager.add_dependencies(to_add);
    manager.execute();
}
