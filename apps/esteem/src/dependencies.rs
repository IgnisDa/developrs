use super::{
    constants::{DEVELOPMENT_KEY, REQUIRED_KEY},
    AddEsteemDevelopmentDependency, AddEsteemRequiredDependency, LibraryError,
    RemoveEsteemDevelopmentDependency, RemoveEsteemRequiredDependency,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EsteemDependencies {
    /// dependencies of the project/workspace
    #[serde(default)]
    pub required: BTreeSet<String>,
    /// devDependencies of the project/workspace
    #[serde(default)]
    pub development: BTreeSet<String>,
}

impl EsteemDependencies {
    pub(crate) fn get_all_dependencies(&self) -> Vec<String> {
        Vec::from_iter(
            self.development
                .iter()
                .chain(self.required.iter())
                .map(String::from),
        )
    }
}

impl Default for EsteemDependencies {
    fn default() -> Self {
        let required = BTreeSet::new();
        let development = BTreeSet::new();
        Self {
            required,
            development,
        }
    }
}

impl AddEsteemRequiredDependency for EsteemDependencies {
    fn add_required_dependency(&mut self, dependency: String) {
        info!("Adding {REQUIRED_KEY} dependency {dependency:?}");
        self.required.insert(dependency);
    }
}

impl AddEsteemDevelopmentDependency for EsteemDependencies {
    fn add_development_dependency(&mut self, dependency: String) {
        info!("Adding {DEVELOPMENT_KEY} dependency {dependency:?}");
        self.development.insert(dependency);
    }
}

impl RemoveEsteemRequiredDependency for EsteemDependencies {
    fn remove_required_dependency(
        &mut self,
        dependency: String,
    ) -> Result<(), LibraryError> {
        info!("Trying to remove {REQUIRED_KEY} dependency {dependency:?}");
        self.required
            .take(&dependency)
            .map(|_| {
                info!("Found and removed {REQUIRED_KEY} dependency {dependency:?} successfully");
            })
            .ok_or_else(|| {
                LibraryError(format!(
                    "Could not find {REQUIRED_KEY} dependency to remove: {dependency:?}"
                ))
            })
    }
}

impl RemoveEsteemDevelopmentDependency for EsteemDependencies {
    fn remove_development_dependency(
        &mut self,
        dependency: String,
    ) -> Result<(), LibraryError> {
        info!("Trying to remove {DEVELOPMENT_KEY} dependency {dependency:?}");
        self.development
            .take(&dependency)
            .map(|_| {
                info!("Found and removed {DEVELOPMENT_KEY} dependency {dependency:?} successfully");
            })
            .ok_or_else(|| {
                LibraryError(format!(
                    "Could not find {DEVELOPMENT_KEY} dependency to remove: {dependency:?}"
                ))
            })
    }
}
