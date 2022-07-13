use indexmap::IndexMap;
use serde_json::{json, to_string_pretty, Value};
use std::{collections::HashMap, fs, path::PathBuf, process};

use crate::commons::{
    constants::{DEPENDENCIES_KEY, DEVELOPMENT_KEY, REQUIRED_KEY},
    lib::Command,
};

#[derive(Debug)]
pub(crate) struct Init {
    projects_file_paths: HashMap<String, PathBuf>,
}

impl Init {
    pub(crate) fn new(projects_file_paths: HashMap<String, PathBuf>) -> Init {
        Init {
            projects_file_paths,
        }
    }
}

impl Command for Init {
    fn execute(&self) {
        for (project_name, project_file_path) in &self.projects_file_paths {
            let s = fs::read_to_string(project_file_path).unwrap();
            let mut project_file_json: IndexMap<String, Value> =
                serde_json::from_str(&s).unwrap();
            if !project_file_json.contains_key(DEPENDENCIES_KEY) {
                project_file_json.insert(
                    DEPENDENCIES_KEY.into(),
                    json!({ REQUIRED_KEY: [], DEVELOPMENT_KEY: []}),
                );
                info!("Writing to file {:?}", project_file_path);
                let to_write = to_string_pretty(&project_file_json).unwrap();
                fs::write(project_file_path, to_write).unwrap();
            } else {
                let dependencies =
                    project_file_json.get(DEPENDENCIES_KEY).unwrap().clone();
                match dependencies {
                    Value::Object(val) => {
                        let development = val.get(DEVELOPMENT_KEY).unwrap_or_else(|| {
                            error!("The {:?} object of project {:?} does not contain a {:?} key", DEPENDENCIES_KEY, project_name, DEVELOPMENT_KEY);
                            process::exit(1);
                        }
                        ).clone();
                        development.as_array().ok_or_else(|| {
                            error!("The {:?} key should be an array", DEVELOPMENT_KEY);
                            process::exit(1);
                        }).unwrap();
                        let required = val.get(REQUIRED_KEY).unwrap_or_else(||{
                            error!("The {:?} object of project {:?} does not contain a {:?} key", DEPENDENCIES_KEY, project_name, REQUIRED_KEY);
                            process::exit(1);
                        }
                        ).clone();
                        required.as_array().ok_or_else(|| {
                            error!("The {:?} key should be an array", REQUIRED_KEY);
                            process::exit(1);
                        }).unwrap();
                        Ok(())
                    }
                    _ => Err(format!("The {:?} should be an object, it is {:?} instead",DEPENDENCIES_KEY, dependencies)),
                }.unwrap();
            }
        }
    }
}
