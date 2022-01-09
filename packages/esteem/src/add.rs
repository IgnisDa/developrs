use crate::{Command, PackageManager, DEPENDENCIES_KEY, DEVELOPMENT_KEY, REQUIRED_KEY};
use indexmap::IndexMap;
use serde_json::Value;
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
        let mut contents: IndexMap<String, Value> =
            serde_json::from_str(&fs::read_to_string(&self.project_path).unwrap())
                .unwrap();
        let mut dependencies = contents.get(DEPENDENCIES_KEY).unwrap().clone();
        let add_to = match self.is_development {
            true => DEVELOPMENT_KEY,
            false => REQUIRED_KEY,
        };
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
        info!(
            "Writing new workspace dependencies to {:?}",
            &self.project_path
        );
        let to_write = serde_json::to_string_pretty(&contents).unwrap();
        fs::write(&self.project_path, to_write).unwrap();
        let mut command: ShellCommand;
        match self.npm_package_manager {
            PackageManager::Npm => {
                command = ShellCommand::new("npm");
            }
            PackageManager::Pnpm => {
                command = ShellCommand::new("pnpm");
            }
            PackageManager::Yarn => {
                command = ShellCommand::new("yarn");
            }
        }
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