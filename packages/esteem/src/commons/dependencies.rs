use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::AddEsteemRequiredDependency;

use super::lib::AddEsteemDevelopmentDependency;

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

impl AddEsteemDevelopmentDependency for EsteemDependencies {
    fn add_development_dependency(&mut self, dependency: String) {
        self.development.as_mut().unwrap().insert(dependency);
    }
}

impl AddEsteemRequiredDependency for EsteemDependencies {
    fn add_required_dependency(&mut self, dependency: String) {
        self.required.as_mut().unwrap().insert(dependency);
    }
}
