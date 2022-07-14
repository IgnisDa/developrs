use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs::{canonicalize, read_to_string},
    path::{Path, PathBuf},
};

use crate::commons::constants::PROJECT_FILE;

use super::{
    dependencies::EsteemDependencies,
    lib::{
        AddEsteemDevelopmentDependency, AddEsteemRequiredDependency, LibraryError,
        WriteDependencies,
    },
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsteemProject {
    /// The name of project, this is unique and should be used as the identifier
    #[serde(skip_serializing, skip_deserializing)]
    pub name: String,

    /// The absolute path of this project's description file
    #[serde(skip_serializing, skip_deserializing)]
    description_file_path: PathBuf,

    /// the dependencies of a project
    dependencies: Option<EsteemDependencies>,

    /// the other miscellaneous keys that we do not care about
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

impl EsteemProject {
    pub fn from_project_path(name: String, path: &Path) -> Result<Self, LibraryError> {
        let description_file_path = canonicalize(path.join(PROJECT_FILE)).unwrap();
        let project_file = read_to_string(&description_file_path);
        match project_file {
            Ok(data) => {
                let mut partial_project: Self = serde_json::from_str(&data).unwrap();
                partial_project.description_file_path = description_file_path;
                partial_project.name = name;
                Ok(partial_project)
            }
            Err(_) => {
                trace!("Unable to find file: {:?}", description_file_path);
                Err(LibraryError)
            }
        }
    }
}

impl AddEsteemRequiredDependency for EsteemProject {
    fn add_required_dependency(&mut self, dependency: String) {
        self.dependencies
            .as_mut()
            .unwrap()
            .add_required_dependency(dependency);
    }
}

impl AddEsteemDevelopmentDependency for EsteemProject {
    fn add_development_dependency(&mut self, dependency: String) {
        self.dependencies
            .as_mut()
            .unwrap()
            .add_development_dependency(dependency);
    }
}

impl WriteDependencies for EsteemProject {
    fn get_path(&self) -> PathBuf {
        self.description_file_path.clone()
    }
}
