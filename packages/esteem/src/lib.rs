mod add;
mod commons;
mod init;
mod install_isolated;
mod remove;
use std::{collections::HashMap, path::PathBuf, process};

pub use commons::{
    constants::{WORKSPACE_FILE, WORKSPACE_IDENTIFIER},
    lib::Workspace,
    utils::get_project_files_for_all_projects,
};
use commons::{lib::Command, utils::get_npm_package_manager};

#[macro_use]
extern crate log;

pub fn perform_add(
    project_path: Option<PathBuf>,
    is_development: bool,
    to_add: Vec<String>,
    is_global: bool,
) {
    let npm_package_manager = get_npm_package_manager().unwrap_or_else(|| {
        error!("A valid lockfile was not found for this project.");
        process::exit(1);
    });
    let a = add::Add::new(
        project_path,
        is_development,
        to_add,
        npm_package_manager,
        is_global,
    );
    a.execute();
}

pub fn perform_init(projects_file_paths: HashMap<String, PathBuf>) {
    let a = init::Init::new(projects_file_paths);
    a.execute();
}

pub fn perform_install_isolated(project_path: Vec<PathBuf>) {
    let a = install_isolated::InstallIsolated::new(project_path);
    a.execute();
}

pub fn perform_remove(
    project_path: Option<PathBuf>,
    to_remove: Vec<String>,
    all_projects: HashMap<String, PathBuf>,
    is_global: bool,
) {
    let npm_package_manager = get_npm_package_manager().unwrap_or_else(|| {
        error!("A valid lockfile was not found for this project.");
        process::exit(1);
    });
    let a = remove::Remove::new(
        project_path,
        to_remove,
        all_projects,
        npm_package_manager,
        is_global,
    );
    a.execute();
}
