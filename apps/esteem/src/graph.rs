use super::{constants::PROJECT_FILE, LibraryError};
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

    pub fn get_projects_with_config_path(&self) -> HashMap<String, PathBuf> {
        let mut ret = HashMap::new();
        for proj in self.get_project_dependencies() {
            let path = self.graph.nodes.get(&proj).unwrap();
            ret.insert(proj, PathBuf::from(&path.data.root).join(PROJECT_FILE));
        }
        ret
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct NxGraph {
    #[serde(default)]
    nodes: HashMap<String, NxNode>,
    #[serde(default)]
    dependencies: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct NxNode {
    #[serde(default)]
    data: NxData,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct NxData {
    #[serde(default)]
    root: String,
}
