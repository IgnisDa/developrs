use super::{
    constants::WORKSPACE_FILE, graph::NxProject, managers::PackageManager,
    project::EsteemProject, workspace::EsteemWorkspace,
};
use std::{collections::HashMap, path::PathBuf};

pub fn display_warning(key: &str, dependency: &str, path: &PathBuf) {
    warn!("{:?} not found in {:?} of {:?}", dependency, key, path);
}

pub fn get_all_project_names() -> Vec<String> {
    let workspace = EsteemWorkspace::from_current_directory();
    match workspace {
        Ok(data) => data.projects.keys().cloned().collect(),
        Err(_) => {
            warn!("This project does not have a {:?} file. The commands will not work as expected. Are you running esteem in the correct directory?", WORKSPACE_FILE);
            vec![]
        }
    }
}

pub fn get_project_dependencies(
    project_name: &String,
    call_script_executor: bool,
) -> Vec<EsteemProject> {
    let mut manager = PackageManager::get_command_executor(call_script_executor).unwrap();
    let path = manager.graph_dependencies(project_name);
    manager.execute_script();
    let project = NxProject::from_path(path).unwrap();
    let projects_names = project.get_project_dependencies();
    let workspace = EsteemWorkspace::from_current_directory().unwrap();
    let projects = projects_names
        .iter()
        .map(|p| workspace.get_project(p.to_string()).cloned().unwrap())
        .collect();
    projects
}

pub fn get_projects_with_config_path(project_name: &String) -> HashMap<String, PathBuf> {
    let mut manager = PackageManager::get_command_executor(true).unwrap();
    let path = manager.graph_dependencies(project_name);
    manager.execute_script();
    let project = NxProject::from_path(path).unwrap();
    project.get_projects_with_config_path()
}
