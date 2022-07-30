use super::{
    constants::{
        DEVELOPMENT_KEY, PACKAGE_JSON_BACKUP_FILE, PACKAGE_JSON_FILE, REQUIRED_KEY,
    },
    managers::PackageManager,
    utils::{display_warning, get_project_dependencies},
    workspace::EsteemWorkspace,
    AddEsteemDevelopmentDependency, AddEsteemRequiredDependency, LibraryError,
    RemoveEsteemDevelopmentDependency, RemoveEsteemRequiredDependency, WriteDependencies,
};
use npm_package_json::Package;
use std::{
    collections::{BTreeMap, BTreeSet},
    env::current_dir,
    fs::rename,
    path::PathBuf,
};

impl WriteDependencies for Package {
    fn get_path(&self) -> PathBuf {
        current_dir().unwrap().join(PACKAGE_JSON_FILE)
    }
}

pub fn perform_add(
    project_name: String,
    to_add: Vec<String>,
    is_development: bool,
    skip_package_manager: bool,
) -> Result<(), LibraryError> {
    let mut workspace = EsteemWorkspace::from_current_directory().unwrap();
    let project = workspace.get_project_mut(project_name).unwrap();
    to_add.iter().for_each(|dependency| {
        if is_development {
            project.add_development_dependency(dependency.into())
        } else {
            project.add_required_dependency(dependency.into())
        }
    });
    project.write_dependencies();
    if !skip_package_manager {
        let mut manager = PackageManager::get_command_executor(true).unwrap();
        if is_development {
            manager.add_development_dependencies(to_add);
        } else {
            manager.add_required_dependencies(to_add);
        }
        manager.execute_command();
    }
    Ok(())
}

pub fn perform_init() -> Result<(), LibraryError> {
    let workspace = EsteemWorkspace::from_current_directory().unwrap();
    workspace.write_dependencies();
    for project in workspace.all_projects_rep {
        project.write_dependencies();
    }
    Ok(())
}

pub fn perform_install_isolated(
    project_names: Vec<String>,
    call_script_executor: bool,
) -> Result<(), LibraryError> {
    let workspace = EsteemWorkspace::from_current_directory().unwrap();
    let mut package_json_file =
        Package::from_path(current_dir().unwrap().join(PACKAGE_JSON_FILE)).unwrap();
    let mut to_install_dev_deps = BTreeSet::new();
    let mut to_install_required_deps = BTreeSet::new();
    info!("Calculating all dependent projects of {project_names:?}");
    project_names.into_iter().for_each(|name| {
        let dependent_projects = get_project_dependencies(&name, call_script_executor);
        info!(
            "{:?} depends on/is depended on by {:?} projects",
            &name,
            &dependent_projects.len()
        );
        dependent_projects.iter().for_each(|p| {
            let deps = p.dependencies.clone();
            to_install_dev_deps.extend(deps.development);
            to_install_required_deps.extend(deps.required);
        });
    });
    to_install_dev_deps.extend(workspace.dependencies.development);
    info!(
        "Number of {DEVELOPMENT_KEY} packages calculated: {:?}",
        to_install_dev_deps.len()
    );
    to_install_required_deps.extend(workspace.dependencies.required);
    info!(
        "Number of {REQUIRED_KEY} packages calculated: {:?}",
        to_install_required_deps.len()
    );
    let workspace_dependencies = package_json_file
        .dependencies
        .into_iter()
        .chain(package_json_file.dev_dependencies.into_iter())
        .collect::<BTreeMap<String, String>>();
    let [filtered_dev_deps, filtered_required_deps] =
        [&to_install_dev_deps, &to_install_required_deps].map(|dep_set| {
            dep_set
                .iter()
                .map(|possible_package| {
                    (
                        possible_package.clone(),
                        workspace_dependencies
                            .get(&possible_package.clone())
                            .cloned()
                            .unwrap_or_else(|| {
                                error!(
                                    "{:?} does not exist in {:?}",
                                    possible_package, PACKAGE_JSON_FILE
                                );
                                panic!("unrecoverable error");
                            }),
                    )
                })
                .collect::<BTreeMap<String, String>>()
        });
    package_json_file.dependencies = filtered_required_deps;
    package_json_file.dev_dependencies = filtered_dev_deps;
    info!("Renaming {PACKAGE_JSON_FILE:?} to {PACKAGE_JSON_BACKUP_FILE:?}",);
    rename(PACKAGE_JSON_FILE, PACKAGE_JSON_BACKUP_FILE).unwrap_or_else(|_| {
        error!("Unable to create backup file, exiting early...");
        panic!("unrecoverable error");
    });
    package_json_file.write_dependencies();
    warn!("Please run your package manager's install command to install the isolated dependencies.");
    Ok(())
}

pub fn perform_remove(
    project_name: String,
    to_remove: Vec<String>,
) -> Result<(), LibraryError> {
    let mut workspace = EsteemWorkspace::from_current_directory().unwrap();
    let project = workspace.get_project_mut(project_name).unwrap();
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
            return Err(LibraryError("will not continue".into()));
        }
    }
    project.write_dependencies();
    let packages_to_remove = workspace.get_dependencies_to_remove(to_remove);
    if packages_to_remove.is_empty() {
        info!(
            "No packages qualifies as removable, will not be calling the package manager"
        );
    } else {
        let mut manager = PackageManager::get_command_executor(true).unwrap();
        manager.remove_dependencies(packages_to_remove);
        manager.execute_command();
    }
    Ok(())
}

pub fn perform_workspace_add(
    to_add: Vec<String>,
    is_development: bool,
    skip_package_manager: bool,
) -> Result<(), LibraryError> {
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
        let mut manager = PackageManager::get_command_executor(true).unwrap();
        if is_development {
            manager.add_development_dependencies(to_add);
        } else {
            manager.add_required_dependencies(to_add);
        }
        manager.execute_command();
    }
    Ok(())
}

pub fn perform_workspace_remove(to_remove: Vec<String>) -> Result<(), LibraryError> {
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
            return Err(LibraryError("will not continue".into()));
        }
    }
    workspace.write_dependencies();
    let packages_to_remove = workspace.get_dependencies_to_remove(to_remove);
    if !packages_to_remove.is_empty() {
        let mut manager = PackageManager::get_command_executor(true).unwrap();
        manager.remove_dependencies(packages_to_remove);
        manager.execute_command();
    }
    Ok(())
}

pub fn utils_get_dependencies(
    project_name: String,
    call_script_executor: bool,
) -> Result<Vec<String>, LibraryError> {
    Ok(
        get_project_dependencies(&project_name, call_script_executor)
            .into_iter()
            .map(|p| p.name)
            .collect::<Vec<_>>(),
    )
}
