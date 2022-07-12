use crate::{
    get_dependencies_from_file, Command, PackageManager, DEPENDENCIES_KEY,
    DEVELOPMENT_KEY, REQUIRED_KEY, WORKSPACE_FILE,
};
use indexmap::IndexMap;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    process::{self, Command as ShellCommand, Stdio},
};

#[derive(Debug)]
pub(crate) struct Remove {
    project_path: Option<PathBuf>,
    to_remove: Vec<String>,
    all_projects: HashMap<String, PathBuf>,
    npm_package_manager: PackageManager,
    is_global: bool,
}

impl Remove {
    pub(crate) fn new(
        project_path: Option<PathBuf>,
        to_remove: Vec<String>,
        all_projects: HashMap<String, PathBuf>,
        npm_package_manager: PackageManager,
        is_global: bool,
    ) -> Self {
        Self {
            project_path,
            all_projects,
            to_remove,
            npm_package_manager,
            is_global,
        }
    }
}

impl Command for Remove {
    fn execute(&self) {
        let mut contents: IndexMap<String, Value> = match self.is_global {
            true => serde_json::from_str(&fs::read_to_string(WORKSPACE_FILE).unwrap())
                .unwrap(),
            false => serde_json::from_str(
                &fs::read_to_string(&self.project_path.clone().unwrap()).unwrap(),
            )
            .unwrap(),
        };
        let write_target = if self.is_global {
            PathBuf::from(WORKSPACE_FILE)
        } else {
            self.project_path.clone().unwrap()
        };
        if let Some((mut required, mut development, mut dependencies)) =
            get_dependencies_from_file(&PathBuf::from(&write_target))
        {
            self.to_remove.iter().for_each(|remove| {
                development.retain(|v| !v.as_str().unwrap().eq(remove))
            });
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
            fs::write(&write_target, to_write).unwrap();
            let mut will_be_removed_from_package_json = HashMap::new();
            self.to_remove.iter().for_each(|f| {
                will_be_removed_from_package_json.insert(f, true);
            });
            for package_name in &self.to_remove {
                for (project_name, project_path) in &self.all_projects {
                    let (required, development, _) =
                        get_dependencies_from_file(project_path).unwrap();
                    if required
                        .iter()
                        .any(|p_n| p_n.as_str().unwrap() == package_name)
                    {
                        warn!(
                            "Found {:?} in {}'s {:?}, won't be removing it!",
                            package_name, project_name, REQUIRED_KEY
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
                            package_name, project_name, DEVELOPMENT_KEY
                        );
                        will_be_removed_from_package_json.insert(package_name, false);
                        break;
                    };
                }
            }
            let are_any_packages_to_be_removed = will_be_removed_from_package_json
                .values()
                .cloned()
                .any(|f| f);
            if !are_any_packages_to_be_removed {
                warn!("No packages to be uninstalled, quitting without calling package manager.");
                return;
            }
            let mut command = match self.npm_package_manager {
                PackageManager::Npm => ShellCommand::new("npm"),
                PackageManager::Pnpm => ShellCommand::new("pnpm"),
                PackageManager::Yarn => ShellCommand::new("yarn"),
            };
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
        } else {
            error!(
                "{:?} does not have a {:?} key",
                self.project_path, DEPENDENCIES_KEY
            );
            process::exit(1);
        };
    }
}
