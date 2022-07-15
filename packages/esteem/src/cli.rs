use super::{
    constants::{DEVELOPMENT_KEY, REQUIRED_KEY},
    init, install_isolated,
    managers::PackageManager,
    utils::display_warning,
    AddEsteemDevelopmentDependency, AddEsteemRequiredDependency, Command,
    EsteemWorkspace, RemoveEsteemDevelopmentDependency, RemoveEsteemRequiredDependency,
    WriteDependencies,
};
use std::{collections::BTreeMap, path::PathBuf, process::exit};

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
            project.add_development_dependency(dependency.into())
        } else {
            project.add_required_dependency(dependency.into())
        }
    });
    project.write_dependencies();
    if !skip_package_manager {
        let mut manager = PackageManager::from_current_directory().unwrap();
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

pub fn perform_remove(project_name: String, to_remove: Vec<String>) {
    let mut workspace = EsteemWorkspace::from_current_directory().unwrap();
    let project = workspace.get_project(project_name).unwrap();
    for dependency in to_remove.iter() {
        let mut should_proceed = false;
        match project.remove_development_dependency(dependency.into()) {
            Ok(_) => should_proceed = true,
            Err(_) => display_warning(DEVELOPMENT_KEY, dependency, &project.get_path()),
        }
        match project.remove_required_dependency(dependency.into()) {
            Ok(_) => should_proceed = true,
            Err(_) => display_warning(REQUIRED_KEY, dependency, &project.get_path()),
        }
        if !should_proceed {
            error!(
                "{:?} not found in {:?}, exiting early without writing to file",
                dependency,
                &project.get_path()
            );
            exit(1);
        }
    }
    project.write_dependencies();
    let packages_to_remove = workspace.get_dependencies_to_remove(to_remove);
    if !packages_to_remove.is_empty() {
        let mut manager = PackageManager::from_current_directory().unwrap();
        manager.remove_dependencies(packages_to_remove);
        manager.execute();
    }
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
        let mut manager = PackageManager::from_current_directory().unwrap();
        manager.add_dependencies(to_add);
        manager.execute();
    }
}

pub fn perform_workspace_remove(to_remove: Vec<String>) {
    let mut workspace = EsteemWorkspace::from_current_directory().unwrap();
    for dependency in to_remove.iter() {
        let mut should_proceed = false;
        match workspace.remove_development_dependency(dependency.into()) {
            Ok(_) => should_proceed = true,
            Err(_) => display_warning(DEVELOPMENT_KEY, dependency, &workspace.get_path()),
        }
        match workspace.remove_required_dependency(dependency.into()) {
            Ok(_) => should_proceed = true,
            Err(_) => display_warning(REQUIRED_KEY, dependency, &workspace.get_path()),
        }
        if !should_proceed {
            error!(
                "{:?} not found in {:?}, exiting early without writing to file",
                dependency,
                &workspace.get_path()
            );
            exit(1);
        }
    }
    workspace.write_dependencies();
    let packages_to_remove = workspace.get_dependencies_to_remove(to_remove);
    if !packages_to_remove.is_empty() {
        let mut manager = PackageManager::from_current_directory().unwrap();
        manager.remove_dependencies(packages_to_remove);
        manager.execute();
    }
}
