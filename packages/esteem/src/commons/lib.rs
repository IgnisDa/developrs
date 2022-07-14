use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::write;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    env::current_dir,
    error::Error,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use super::constants::{workspace_file, WORKSPACE_FILE};

#[derive(Debug)]
pub struct LibraryError;

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Library error")
    }
}

impl Error for LibraryError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EsteemDependencies {
    /// dependencies of the project/workspace
    required: Option<BTreeSet<String>>,
    /// devDependencies of the project/workspace
    development: Option<BTreeSet<String>>,
}

impl Default for EsteemDependencies {
    fn default() -> Self {
        let required = Some(BTreeSet::new());
        let development = Some(BTreeSet::new());
        Self {
            required,
            development,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EsteemWorkspace {
    /// The complete path to this project
    #[serde(default = "workspace_file", skip_serializing)]
    path: PathBuf,

    /// a mapping of projects to their paths
    pub projects: HashMap<String, PathBuf>,

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
        let projects = HashMap::new();
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

pub trait Command {
    fn execute(&self);
}

#[derive(Debug)]
pub enum PackageManager {
    Yarn,
    Pnpm,
    Npm,
}

/// Used to add a required dependency to a project or workspace
pub trait AddEsteemRequiredDependency {
    fn add_required_dependency(&mut self, dependency: String);
}

impl AddEsteemRequiredDependency for EsteemWorkspace {
    fn add_required_dependency(&mut self, dependency: String) {
        self.dependencies
            .as_mut()
            .unwrap()
            .required
            .as_mut()
            .unwrap()
            .insert(dependency);
    }
}

/// Used to add a development dependency to a project or workspace
pub trait AddEsteemDevelopmentDependency {
    fn add_development_dependency(&mut self, dependency: String);
}

impl AddEsteemDevelopmentDependency for EsteemWorkspace {
    fn add_development_dependency(&mut self, dependency: String) {
        self.dependencies
            .as_mut()
            .unwrap()
            .development
            .as_mut()
            .unwrap()
            .insert(dependency);
    }
}

/// Used to write dependencies to a file
pub trait WriteDependencies {
    fn write_dependencies(&self);
}

impl WriteDependencies for EsteemWorkspace {
    fn write_dependencies(&self) {
        let to_write = serde_json::to_string_pretty(self).unwrap();
        write(&self.path, to_write).unwrap();
    }
}
