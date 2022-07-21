use super::{
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
        self.required.insert(dependency);
    }
}

impl AddEsteemDevelopmentDependency for EsteemDependencies {
    fn add_development_dependency(&mut self, dependency: String) {
        self.development.insert(dependency);
    }
}

impl RemoveEsteemRequiredDependency for EsteemDependencies {
    fn remove_required_dependency(
        &mut self,
        dependency: String,
    ) -> Result<(), LibraryError> {
        self.required
            .take(&dependency)
            .map(|_| ())
            .ok_or(LibraryError)
    }
}

impl RemoveEsteemDevelopmentDependency for EsteemDependencies {
    fn remove_development_dependency(
        &mut self,
        dependency: String,
    ) -> Result<(), LibraryError> {
        self.development
            .take(&dependency)
            .map(|_| ())
            .ok_or(LibraryError)
    }
}
