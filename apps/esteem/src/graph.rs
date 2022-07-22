use super::LibraryError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NxProject {
    #[serde(default)]
    graph: NxGraph,
}

impl NxProject {
    pub fn from_path(path: PathBuf) -> Result<Self, LibraryError> {
        let workspace_file = read_to_string(&path).unwrap();
        Ok(serde_json::from_str(&workspace_file).unwrap())
    }

    pub fn get_project_dependencies(&self) -> Vec<String> {
        self.graph.dependencies.keys().cloned().collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct NxGraph {
    #[serde(default)]
    dependencies: HashMap<String, Value>,
}
