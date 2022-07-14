use super::{
    constants::{workspace_file, WORKSPACE_FILE},
    dependencies::EsteemDependencies,
    project::EsteemProject,
    AddEsteemDevelopmentDependency, AddEsteemRequiredDependency, LibraryError,
    WriteDependencies,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    env::current_dir,
    fs::read_to_string,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EsteemWorkspace {
    /// The complete path to this project
    #[serde(default = "workspace_file", skip_serializing)]
    path: PathBuf,

    /// a mapping of projects to their paths
    // TODO: Remove the pub part
    pub projects: BTreeMap<String, PathBuf>,

    #[serde(skip_serializing, skip_deserializing)]
    pub all_projects_rep: Vec<EsteemProject>,

    /// the dependencies of a project
    #[serde(default)]
    dependencies: EsteemDependencies,

    /// the other miscellaneous keys that we do not care about
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

impl EsteemWorkspace {
    pub fn from_current_directory() -> Result<Self, LibraryError> {
        let workspace_file =
            read_to_string(Path::new(&current_dir().unwrap()).join(WORKSPACE_FILE));
        match workspace_file {
            Ok(data) => {
                let mut work: Self = serde_json::from_str(&data).unwrap();
                let projects_internal = work
                    .projects
                    .iter()
                    .map(|(name, path)| {
                        EsteemProject::from_project_path(name.clone(), path).unwrap()
                    })
                    .collect();
                work.all_projects_rep = projects_internal;
                Ok(work)
            }
            Err(_) => {
                trace!("Unable to find file: {:?}", WORKSPACE_FILE);
                Err(LibraryError)
            }
        }
    }

    pub fn get_project(
        &mut self,
        project_name: String,
    ) -> Result<&mut EsteemProject, LibraryError> {
        let project = self
            .all_projects_rep
            .iter_mut()
            .find(|p| p.name == project_name);
        project.ok_or(LibraryError)
    }
}

impl AddEsteemRequiredDependency for EsteemWorkspace {
    fn add_required_dependency(&mut self, dependency: String) {
        self.dependencies.add_required_dependency(dependency);
    }
}

impl AddEsteemDevelopmentDependency for EsteemWorkspace {
    fn add_development_dependency(&mut self, dependency: String) {
        self.dependencies.add_development_dependency(dependency);
    }
}

impl WriteDependencies for EsteemWorkspace {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}
