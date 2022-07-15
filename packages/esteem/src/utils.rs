use super::{constants::WORKSPACE_FILE, workspace::EsteemWorkspace};
use std::path::PathBuf;

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
