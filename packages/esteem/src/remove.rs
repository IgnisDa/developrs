use crate::{Command, PackageManager, DEPENDENCIES_KEY, DEVELOPMENT_KEY, REQUIRED_KEY};
use indexmap::IndexMap;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    process::{Command as ShellCommand, Stdio},
};

#[derive(Debug)]
pub(crate) struct Remove {
    project_path: PathBuf,
    to_remove: Vec<String>,
    all_projects: HashMap<String, PathBuf>,
    npm_package_manager: PackageManager,
}

impl Remove {
    pub(crate) fn new(
        project_path: PathBuf,
        to_remove: Vec<String>,
        all_projects: HashMap<String, PathBuf>,
        npm_package_manager: PackageManager,
    ) -> Self {
        Self {
            project_path,
            all_projects,
            to_remove,
            npm_package_manager,
        }
    }
}

impl Command for Remove {
    fn execute(&self) {
        let mut contents: IndexMap<String, Value> =
            serde_json::from_str(&fs::read_to_string(&self.project_path).unwrap())
                .unwrap();
        let mut dependencies = contents.get(DEPENDENCIES_KEY).unwrap().clone();
        let d = dependencies[DEVELOPMENT_KEY].to_owned();
        let mut development = d.as_array().unwrap().clone();
        let r = dependencies[REQUIRED_KEY].to_owned();
        let mut required = r.as_array().unwrap().clone();
        self.to_remove
            .iter()
            .for_each(|remove| development.retain(|v| !v.as_str().unwrap().eq(remove)));
        self.to_remove
            .iter()
            .for_each(|remove| required.retain(|v| !v.as_str().unwrap().eq(remove)));
        dependencies[REQUIRED_KEY] = json!(required);
        dependencies[DEVELOPMENT_KEY] = json!(development);
        contents.insert(DEPENDENCIES_KEY.into(), dependencies);
        info!(
            "Writing new workspace dependencies to {:?}",
            &self.project_path
        );
        let to_write = serde_json::to_string_pretty(&contents).unwrap();
        fs::write(&self.project_path, to_write).unwrap();

        let mut will_be_removed_from_package_json = HashMap::new();
        self.to_remove.iter().for_each(|f| {
            will_be_removed_from_package_json.insert(f, true);
        });
        for package_name in &self.to_remove {
            for (project_name, project_path) in &self.all_projects {
                let contents: IndexMap<String, Value> =
                    serde_json::from_str(&fs::read_to_string(project_path).unwrap())
                        .unwrap();
                let dependencies = contents.get(DEPENDENCIES_KEY).unwrap().clone();
                let required = dependencies[REQUIRED_KEY].as_array().unwrap();
                let development = dependencies[DEVELOPMENT_KEY].as_array().unwrap();
                if required
                    .iter()
                    .any(|p_n| p_n.as_str().unwrap() == package_name)
                {
                    warn!(
                        "Found {:?} in {}'s {:?}, won't be removing it!",
                        package_name, REQUIRED_KEY, project_name
                    );
                    will_be_removed_from_package_json.insert(package_name, false);
                    break;
                };
                if development
                    .iter()
                    .any(|p_n| p_n.as_str().unwrap() == package_name)
                {
                    warn!(
                        "Found {:?} in {}'s {:?}, won't be removing it!",
                        package_name, DEVELOPMENT_KEY, project_name
                    );
                    will_be_removed_from_package_json.insert(package_name, false);
                    break;
                };
            }
        }
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
        command.arg("remove");
        will_be_removed_from_package_json.iter().for_each(
            |(&package_name, &will_be_removed)| {
                if will_be_removed {
                    command.arg(package_name);
                }
            },
        );
        let mut output = command
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to execute command...");
        output.wait().unwrap();
    }
}
