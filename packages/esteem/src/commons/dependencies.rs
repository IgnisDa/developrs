use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::AddEsteemRequiredDependency;

use super::lib::AddEsteemDevelopmentDependency;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EsteemDependencies {
    /// dependencies of the project/workspace
    #[serde(default)]
    required: BTreeSet<String>,
    /// devDependencies of the project/workspace
    #[serde(default)]
    development: BTreeSet<String>,
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

impl AddEsteemDevelopmentDependency for EsteemDependencies {
    fn add_development_dependency(&mut self, dependency: String) {
        self.development.insert(dependency);
    }
}

impl AddEsteemRequiredDependency for EsteemDependencies {
    fn add_required_dependency(&mut self, dependency: String) {
        self.required.insert(dependency);
    }
}
