use super::constants::{DEPENDENCIES_KEY, DEVELOPMENT_KEY, REQUIRED_KEY};
use serde_json::Value;
use std::{collections::BTreeMap, fs::read_to_string, path::PathBuf};

pub fn get_dependencies_from_file(
    file_path: &PathBuf,
) -> Option<(Vec<Value>, Vec<Value>, Value)> {
    let contents: BTreeMap<String, Value> =
        serde_json::from_str(&read_to_string(file_path).unwrap()).unwrap();
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

pub fn display_warning(key: &str, dependency: &str, path: &PathBuf) {
    warn!("{:?} not found in {:?} of {:?}", dependency, key, path);
}
