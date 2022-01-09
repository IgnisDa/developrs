use indexmap::IndexMap;
use serde_json::Value;

use crate::{
    Command, Package, DEPENDENCIES_KEY, DEVELOPMENT_KEY, PACKAGE_JSON_BACKUP_FILE,
    PACKAGE_JSON_FILE, REQUIRED_KEY,
};
use std::{collections::HashMap, fs, path::PathBuf, process};

pub(crate) struct InstallIsolated {
    project_path: PathBuf,
}

impl InstallIsolated {
    pub(crate) fn new(project_path: PathBuf) -> InstallIsolated {
        InstallIsolated { project_path }
    }
}

impl Command for InstallIsolated {
    fn execute(&self) {
        let contents: IndexMap<String, Value> =
            serde_json::from_str(&fs::read_to_string(&self.project_path).unwrap())
                .unwrap();
        let workspace = Package::default();
        let mut dependencies = HashMap::new();
        dependencies.extend(workspace.dependencies);
        dependencies.extend(workspace.dev_dependencies);
        let project_dependencies = contents.get(DEPENDENCIES_KEY).unwrap().clone();
        let to_install_required_deps = project_dependencies[REQUIRED_KEY]
            .as_array()
            .unwrap()
            .clone();
        let to_install_development_deps = project_dependencies[DEVELOPMENT_KEY]
            .as_array()
            .unwrap()
            .clone();
        let filtered_dev_deps = to_install_development_deps
            .iter()
            .map(|possible_package| {
                (
                    possible_package.as_str().unwrap().to_string(),
                    dependencies
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
                        .to_string(),
                )
            })
            .collect::<HashMap<String, String>>();
        let filtered_required_deps = to_install_required_deps
            .iter()
            .map(|possible_package| {
                (
                    possible_package.as_str().unwrap().to_string(),
                    dependencies
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
                        .to_string(),
                )
            })
            .collect::<HashMap<String, String>>();
        let new_package_json_struct =
            Package::from_hashmaps(filtered_required_deps, filtered_dev_deps);
        info!(
            "Renaming file {:?} to {:?}",
            PACKAGE_JSON_FILE, PACKAGE_JSON_BACKUP_FILE
        );
        fs::rename(PACKAGE_JSON_FILE, PACKAGE_JSON_BACKUP_FILE).unwrap_or_else(|_| {
            error!("Unable to rename file");
        });
        info!("Writing to file {:?}", PACKAGE_JSON_FILE);
        let to_write = serde_json::to_string_pretty(&new_package_json_struct).unwrap();
        fs::write(PACKAGE_JSON_FILE, to_write).unwrap();
        println!("\n\nNOTE: Please run your package manager's install command to complete installing the dependencies.\n\n");
    }
}
