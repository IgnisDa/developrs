use crate::commons::{
    constants::{
        DEPENDENCIES_KEY, DEVELOPMENT_DEPENDENCIES_KEY, PACKAGE_JSON_BACKUP_FILE,
        PACKAGE_JSON_FILE, WORKSPACE_FILE,
    },
    lib::Command,
    utils::get_dependencies_from_file,
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::PathBuf,
    process,
};

pub(crate) struct InstallIsolated {
    project_paths: Vec<PathBuf>,
}

impl InstallIsolated {
    pub(crate) fn new(project_paths: Vec<PathBuf>) -> InstallIsolated {
        InstallIsolated { project_paths }
    }
}

impl Command for InstallIsolated {
    fn execute(&self) {
        let mut package_file: BTreeMap<String, Value> = serde_json::from_str(
            &fs::read_to_string(PACKAGE_JSON_FILE).unwrap_or_else(|_| {
                error!("Unable to read file: {:?}", PACKAGE_JSON_FILE);
                process::exit(1);
            }),
        )
        .unwrap();
        let package_file_required_deps = package_file
            .get(DEPENDENCIES_KEY)
            .cloned()
            .unwrap_or_else(|| json!({}))
            .as_object()
            .unwrap()
            .clone();
        let package_file_dev_deps = package_file
            .get(DEVELOPMENT_DEPENDENCIES_KEY)
            .cloned()
            .unwrap_or_else(|| json!({}))
            .as_object()
            .unwrap()
            .clone();
        let mut workspace_dependencies = HashMap::new();
        workspace_dependencies.extend(package_file_required_deps);
        workspace_dependencies.extend(package_file_dev_deps);
        let mut to_install_development_deps = Vec::new();
        let mut to_install_required_deps = Vec::new();
        for project_path in &self.project_paths {
            let (project_required, project_dev, _) =
                get_dependencies_from_file(project_path).unwrap();
            to_install_required_deps.extend(project_required);
            to_install_development_deps.extend(project_dev);
        }
        // handle global dependencies in the workspace file
        if let Some((global_required, global_dev, _)) =
            get_dependencies_from_file(&PathBuf::from(WORKSPACE_FILE))
        {
            to_install_required_deps.extend(global_required);
            to_install_development_deps.extend(global_dev);
        }
        let filtered_dev_deps = to_install_development_deps
            .iter()
            .map(|possible_package| {
                (
                    possible_package.as_str().unwrap().to_string(),
                    workspace_dependencies
                        .get(possible_package.as_str().unwrap())
                        .unwrap_or_else(|| {
                            error!(
                                "{:?} does not exist in {:?}",
                                possible_package.as_str().unwrap().to_string(),
                                PACKAGE_JSON_FILE
                            );
                            process::exit(1);
                        })
                        .as_str()
                        .unwrap()
                        .to_string(),
                )
            })
            .collect::<HashMap<String, String>>();
        let filtered_required_deps = to_install_required_deps
            .iter()
            .map(|possible_package| {
                (
                    possible_package.as_str().unwrap().to_string(),
                    workspace_dependencies
                        .get(possible_package.as_str().unwrap())
                        .unwrap_or_else(|| {
                            error!(
                                "{:?} does not exist in {:?}",
                                possible_package.as_str().unwrap().to_string(),
                                PACKAGE_JSON_FILE
                            );
                            process::exit(1);
                        })
                        .as_str()
                        .unwrap()
                        .to_string(),
                )
            })
            .collect::<HashMap<String, String>>();
        package_file.insert(
            DEVELOPMENT_DEPENDENCIES_KEY.into(),
            json!(filtered_dev_deps),
        );
        package_file.insert(DEPENDENCIES_KEY.into(), json!(filtered_required_deps));
        info!(
            "Renaming file {:?} to {:?}",
            PACKAGE_JSON_FILE, PACKAGE_JSON_BACKUP_FILE
        );
        fs::rename(PACKAGE_JSON_FILE, PACKAGE_JSON_BACKUP_FILE).unwrap_or_else(|_| {
            error!("Unable to rename file");
        });
        info!("Writing to file {:?}", PACKAGE_JSON_FILE);
        let to_write = serde_json::to_string_pretty(&package_file).unwrap();
        fs::write(PACKAGE_JSON_FILE, to_write).unwrap();
        println!("\n\nNOTE: Please run your package manager's install command to complete installing the dependencies.\n\n");
    }
}
