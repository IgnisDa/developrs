use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    env::current_dir,
    fs::read_to_string,
    path::{Path, PathBuf},
};
use uuid::Uuid;

use crate::commons::constants::PROJECT_FILE;

use super::{
    dependencies::EsteemDependencies,
    lib::{
        AddEsteemDevelopmentDependency, AddEsteemRequiredDependency, LibraryError,
        WriteDependencies,
    },
};

fn deserialize_project_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    Ok(PathBuf::from(&buf).join(PROJECT_FILE))
}

fn serialize_project_path<S>(x: &PathBuf, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let parent = Path::new(x).parent().unwrap().to_str().unwrap();
    s.serialize_str(parent)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsteemProject {
    // a unique ID associated with this project
    #[serde(default = "Uuid::new_v4", skip_serializing)]
    id: Uuid,

    /// The complete path to this project description file
    #[serde(
        deserialize_with = "deserialize_project_path",
        serialize_with = "serialize_project_path"
    )]
    source_root: PathBuf,

    /// the dependencies of a project
    dependencies: Option<EsteemDependencies>,

    /// the other miscellaneous keys that we do not care about
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

impl EsteemProject {
    pub fn from_project_path(path: &PathBuf) -> Result<Self, LibraryError> {
        let project_file = read_to_string(
            Path::new(&current_dir().unwrap())
                .join(path)
                .join(PROJECT_FILE),
        );
        match project_file {
            Ok(data) => Ok(serde_json::from_str(&data).unwrap()),
            Err(_) => {
                trace!("Unable to find file: {:?}", path);
                Err(LibraryError)
            }
        }
    }
}

impl AddEsteemRequiredDependency for EsteemProject {
    fn add_required_dependency(&mut self, dependency: String) {
        self.dependencies
            .as_mut()
            .unwrap()
            .add_required_dependency(dependency);
    }
}

impl AddEsteemDevelopmentDependency for EsteemProject {
    fn add_development_dependency(&mut self, dependency: String) {
        self.dependencies
            .as_mut()
            .unwrap()
            .add_development_dependency(dependency);
    }
}

impl WriteDependencies for EsteemProject {
    fn get_path(&self) -> PathBuf {
        self.source_root.clone()
    }
}
