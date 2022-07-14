use crate::commons::{
    constants::{DEPENDENCIES_KEY, DEVELOPMENT_KEY, REQUIRED_KEY},
    lib::{Command, PackageManager},
};
use indexmap::IndexMap;
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    process::{Command as ShellCommand, Stdio},
};

#[derive(Debug)]
pub(crate) struct Add {
    project_path: PathBuf,
    is_development: bool,
    to_add: Vec<String>,
    npm_package_manager: PackageManager,
}

impl Add {
    pub(crate) fn new(
        project_path: PathBuf,
        is_development: bool,
        to_add: Vec<String>,
        npm_package_manager: PackageManager,
    ) -> Self {
        Self {
            project_path,
            is_development,
            to_add,
            npm_package_manager,
        }
    }
}

impl Command for Add {
    fn execute(&self) {
        let mut contents: IndexMap<String, Value> = serde_json::from_str(
            &fs::read_to_string(&self.project_path.clone()).unwrap(),
        )
        .unwrap();
        let add_to = match self.is_development {
            true => DEVELOPMENT_KEY,
            false => REQUIRED_KEY,
        };
        // if user's first time running this, will be `None`
        let maybe_dependencies = contents.get(DEPENDENCIES_KEY).cloned();
        match maybe_dependencies {
            Some(mut dependencies) => {
                self.to_add.iter().for_each(|f| {
                    if dependencies[add_to]
                        .as_array()
                        .unwrap()
                        .contains(&serde_json::json!(f))
                    {
                        warn!(
                            "Dependency {:?} already exists in {:?}. Skipping...",
                            f, &self.project_path
                        );
                    } else {
                        info!("Dependency {:?} added to {:?}.", f, &self.project_path);
                        dependencies[add_to]
                            .as_array_mut()
                            .unwrap()
                            .push(serde_json::json!(f))
                    }
                });
                let mut sorted_dependencies = dependencies[add_to]
                    .as_array_mut()
                    .unwrap()
                    .iter()
                    .map(|f| f.as_str().unwrap().to_string())
                    .collect::<Vec<String>>();
                sorted_dependencies.sort_by_key(|a| a.to_lowercase());
                dependencies[add_to] = sorted_dependencies.into();
                contents.insert(DEPENDENCIES_KEY.into(), dependencies);
            }
            None => {
                let all_to_add = self.to_add.to_vec();
                let mut dependencies = IndexMap::new();
                if self.is_development {
                    dependencies.insert(DEVELOPMENT_KEY, all_to_add);
                    dependencies.insert(REQUIRED_KEY, vec![]);
                } else {
                    dependencies.insert(REQUIRED_KEY, all_to_add);
                    dependencies.insert(DEVELOPMENT_KEY, vec![]);
                }
                contents.insert(DEPENDENCIES_KEY.into(), json!(dependencies));
            }
        }
        info!(
            "Writing new workspace dependencies to {:?}",
            &self.project_path
        );
        let to_write = serde_json::to_string_pretty(&contents).unwrap();
        fs::write(&self.project_path.clone(), to_write).unwrap();
        let mut command = match self.npm_package_manager {
            PackageManager::Npm => ShellCommand::new("npm"),
            PackageManager::Pnpm => ShellCommand::new("pnpm"),
            PackageManager::Yarn => ShellCommand::new("yarn"),
        };
        if matches!(
            self.npm_package_manager,
            PackageManager::Npm | PackageManager::Pnpm
        ) {
            command.arg("install");
        } else if matches!(self.npm_package_manager, PackageManager::Yarn) {
            command.arg("add");
        }
        if self.is_development {
            command.arg("-D");
        }
        self.to_add.iter().for_each(|f| {
            command.arg(f);
        });
        info!("Installing package(s) {:?} for you", &self.to_add);
        let mut output = command
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to execute command...");
        output.wait().unwrap();
    }
}
