use super::{
    constants::{workspace_file, WORKSPACE_FILE},
    dependencies::EsteemDependencies,
    project::EsteemProject,
    AddEsteemDevelopmentDependency, AddEsteemRequiredDependency, LibraryError,
    RemoveEsteemDevelopmentDependency, RemoveEsteemRequiredDependency, WriteDependencies,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashSet},
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
    pub(crate) projects: BTreeMap<String, PathBuf>,

    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) all_projects_rep: Vec<EsteemProject>,

    /// the dependencies of a project
    #[serde(default)]
    pub(crate) dependencies: EsteemDependencies,

    /// the other miscellaneous keys that we do not care about
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

impl EsteemWorkspace {
    pub(crate) fn from_current_directory() -> Result<Self, LibraryError> {
        let workspace_file =
            read_to_string(Path::new(&current_dir().unwrap()).join(WORKSPACE_FILE));
        match workspace_file {
            Ok(data) => {
                let mut work: Self = serde_json::from_str(&data).unwrap();
                work.all_projects_rep = work
                    .projects
                    .iter()
                    .flat_map(|(name, path)| {
                        trace!("Processing project: {name:?}");
                        EsteemProject::from_project_path(name.clone(), path)
                    })
                    .collect();
                Ok(work)
            }
            Err(_) => {
                error!("Unable to find file: {:?}", WORKSPACE_FILE);
                Err(LibraryError(format!(
                    "Could not find file: {WORKSPACE_FILE:?}"
                )))
            }
        }
    }

    pub(crate) fn get_project_mut(
        &mut self,
        project_name: String,
    ) -> Result<&mut EsteemProject, LibraryError> {
        let project = self
            .all_projects_rep
            .iter_mut()
            .find(|p| p.name == project_name);
        project.ok_or_else(|| {
            LibraryError(format!(
                "Could not find a project associated with name: {project_name:?}"
            ))
        })
    }

    pub(crate) fn get_project(
        &self,
        project_name: String,
    ) -> Result<&EsteemProject, LibraryError> {
        let project = self
            .all_projects_rep
            .iter()
            .find(|p| p.name == project_name);
        project.ok_or_else(|| {
            LibraryError(format!(
                "Could not find a project associated with name: {project_name:?}"
            ))
        })
    }

    /// returns all dependencies of this project (project and workspace scoped)
    fn get_all_dependencies(&self) -> HashSet<String> {
        let workspace_deps = self.dependencies.get_all_dependencies();
        let projects_deps = self
            .all_projects_rep
            .iter()
            .flat_map(|p| p.dependencies.get_all_dependencies());
        let all_deps_vec = workspace_deps.into_iter().chain(projects_deps.into_iter());
        HashSet::from_iter(all_deps_vec)
    }

    /// returns whether a dependency is a part of this workspace by scanning workspace and
    /// all project files
    pub fn is_dependency_present(&self, dependency: &str) -> bool {
        let all_deps = self.get_all_dependencies();
        all_deps.contains(dependency)
    }

    /// given a list of dependencies to remove, this returns all of the dependencies that
    /// should be actually removed (and not the ones that are present in other projects)
    pub fn get_dependencies_to_remove(&self, to_remove: Vec<String>) -> Vec<String> {
        to_remove
            .into_iter()
            .map(|dependency| {
                (
                    dependency.to_string(),
                    self.is_dependency_present(&dependency),
                )
            })
            .filter(|(_, t)| !*t)
            .map(|(dep, _)| dep)
            .collect::<Vec<String>>()
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

impl RemoveEsteemRequiredDependency for EsteemWorkspace {
    fn remove_required_dependency(
        &mut self,
        dependency: String,
    ) -> Result<(), LibraryError> {
        self.dependencies.remove_required_dependency(dependency)
    }
}

impl RemoveEsteemDevelopmentDependency for EsteemWorkspace {
    fn remove_development_dependency(
        &mut self,
        dependency: String,
    ) -> Result<(), LibraryError> {
        self.dependencies.remove_development_dependency(dependency)
    }
}
