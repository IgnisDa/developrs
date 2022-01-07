use crate::Command;
use indexmap::IndexMap;
use serde_json::{json, to_string_pretty, Value};
use std::{collections::HashMap, fs, path::PathBuf, process};

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
            if !project_file_json.contains_key("dependencies") {
                project_file_json.insert(
                    "dependencies".into(),
                    json!({ "required": [], "development": []}),
                );
                // write it back to the file
                info!("Writing to file {:?}", project_file_json);
                let to_write = to_string_pretty(&project_file_json).unwrap();
                fs::write(project_file_path, to_write).unwrap();
            } else {
                let g = project_file_json.get("dependencies").unwrap().clone();
                match g {
                    Value::Object(val) => {
                        let development = val.get("development").unwrap_or_else(|| {
                            error!("The `dependencies` object of project '{}' does not contain a `development` key", project_name);
                            process::exit(1);
                        }
                        ).clone();
                        development.as_array().ok_or_else(|| {
                            error!("The `development` key should be an array");
                            process::exit(1);
                        }).unwrap();
                        let required = val.get("required").unwrap_or_else(||{
                            error!("The `dependencies` object of project '{}' does not contain a `required` key", project_name);
                            process::exit(1);
                        }
                        ).clone();
                        required.as_array().ok_or_else(|| {
                            error!("The `required` key should be an array");
                            process::exit(1);
                        }).unwrap();
                        Ok(())
                    }
                    _ => Err(format!("The `dependencies` should be an object, it is '{}' instead", g)),
                }.unwrap();
            }
        }
    }
}
