use std::{
    collections::BTreeMap,
    env::current_dir,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    constants::{workspace_file, WORKSPACE_FILE},
    dependencies::EsteemDependencies,
    lib::{
        AddEsteemDevelopmentDependency, AddEsteemRequiredDependency, LibraryError,
        WriteDependencies,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EsteemWorkspace {
    /// The complete path to this project
    #[serde(default = "workspace_file", skip_serializing)]
    path: PathBuf,

    /// a mapping of projects to their paths
    pub projects: BTreeMap<String, PathBuf>,

    /// the dependencies of a project
    dependencies: Option<EsteemDependencies>,

    /// the other miscellaneous keys that we do not care about
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

impl EsteemWorkspace {
    pub fn from_current_directory() -> Result<Self, LibraryError> {
        let workspace_file =
            read_to_string(Path::new(&current_dir().unwrap()).join(WORKSPACE_FILE));
        match workspace_file {
            Ok(data) => Ok(serde_json::from_str(&data).unwrap()),
            Err(_) => {
                trace!("Unable to find file: {:?}", WORKSPACE_FILE);
                Err(LibraryError)
            }
        }
    }
}

impl Default for EsteemWorkspace {
    fn default() -> Self {
        let projects = BTreeMap::new();
        let dependencies = Some(EsteemDependencies::default());
        let path = PathBuf::default();
        let other = BTreeMap::default();
        Self {
            path,
            projects,
            dependencies,
            other,
        }
    }
}

impl AddEsteemRequiredDependency for EsteemWorkspace {
    fn add_required_dependency(&mut self, dependency: String) {
        self.dependencies
            .as_mut()
            .unwrap()
            .add_required_dependency(dependency);
    }
}

impl AddEsteemDevelopmentDependency for EsteemWorkspace {
    fn add_development_dependency(&mut self, dependency: String) {
        self.dependencies
            .as_mut()
            .unwrap()
            .add_development_dependency(dependency);
    }
}

impl WriteDependencies for EsteemWorkspace {
    fn write_dependencies(&self) {
        let to_write = serde_json::to_string_pretty(self).unwrap();
        write(&self.path, to_write).unwrap();
    }
}
