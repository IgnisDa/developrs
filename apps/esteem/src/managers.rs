use super::LibraryError;
use duct::{cmd, Expression};
use once_cell::sync::Lazy;
use std::{
    env::current_dir,
    fs::read_dir,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use tempfile::NamedTempFile;

#[derive(Debug)]
pub struct CommandExecutor {
    /// the installation subcommand of the package manager
    install: String,
    /// the remove subcommand of the package manager
    remove: String,
    /// the script execution binary
    script_executor: String,
    /// the command that executes normal scripts
    command_executor: String,
    /// the command that has to be executed
    command_to_execute: Vec<String>,
    /// whether to directly call the command executor without using `script_executor`
    call_script_executor: bool,
}

impl CommandExecutor {
    fn new(
        install: String,
        remove: String,
        script_executor: String,
        command_executor: String,
        call_script_executor: bool,
    ) -> Self {
        Self {
            install,
            remove,
            script_executor,
            command_executor,
            command_to_execute: vec![],
            call_script_executor,
        }
    }

    pub fn add_dependencies(&mut self, to_add: Vec<String>) {
        self.command_to_execute.push(self.install.clone());
        to_add.into_iter().for_each(|f| {
            self.command_to_execute.push(f);
        });
    }

    pub fn remove_dependencies(&mut self, to_remove: Vec<String>) {
        self.command_to_execute.push(self.remove.clone());
        to_remove.into_iter().for_each(|f| {
            self.command_to_execute.push(f);
        });
    }

    pub fn graph_dependencies(&mut self, project_name: &String) -> PathBuf {
        let file = NamedTempFile::new().unwrap();
        let path = format!("{}.json", file.path().as_os_str().to_str().unwrap());
        self.command_to_execute.extend([
            "nx".into(),
            "graph".into(),
            "--file".into(),
            path.clone(),
            "--focus".into(),
            project_name.to_owned(),
        ]);
        PathBuf::from(&path)
    }

    pub fn execute_command(self) {
        let command = cmd(&self.command_executor, &self.command_to_execute);
        self.execute(command);
    }

    pub fn execute_script(self) {
        let command = match self.call_script_executor {
            true => cmd(&self.script_executor, &self.command_to_execute),
            false => {
                let program = self.command_to_execute[..1].get(0).unwrap();
                let args = &self.command_to_execute[1..];
                cmd(program, args)
            }
        };
        self.execute(command);
    }

    fn execute(self, command: Expression) {
        info!("Calling command: {command:?}");
        let reader = command.stderr_to_stdout().reader().unwrap();
        let lines = BufReader::new(reader).lines();
        for _line in lines {}
    }
}

#[derive(Clone)]
pub struct PackageManager {
    /// the installation subcommand of the package manager
    install: String,
    /// the remove subcommand of the package manager
    remove: String,
    /// the script execution binary
    script_executor: String,
    /// the command that executes normal scripts
    command_executor: String,
}

impl PackageManager {
    pub fn get_command_executor(
        call_script_executor: bool,
    ) -> Result<CommandExecutor, LibraryError> {
        let dir = read_dir(current_dir().unwrap()).unwrap();
        for file in dir {
            let executor =
                match file.unwrap().file_name().to_os_string().to_str().unwrap() {
                    "yarn.lock" => {
                        let pm = YARN_PACKAGE_MANAGER.clone();
                        CommandExecutor::new(
                            pm.install,
                            pm.remove,
                            pm.script_executor,
                            pm.command_executor,
                            call_script_executor,
                        )
                    }
                    "pnpm-lock.yaml" => {
                        let pm = PNPM_PACKAGE_MANAGER.clone();
                        CommandExecutor::new(
                            pm.install,
                            pm.remove,
                            pm.script_executor,
                            pm.command_executor,
                            call_script_executor,
                        )
                    }
                    "package-lock.json" => {
                        let pm = NPM_PACKAGE_MANAGER.clone();
                        CommandExecutor::new(
                            pm.install,
                            pm.remove,
                            pm.script_executor,
                            pm.command_executor,
                            call_script_executor,
                        )
                    }
                    _ => continue,
                };
            return Ok(executor);
        }
        Err(LibraryError("Could not guess an appropriate NPM package manager. Only `NPM`, `YARN` and `PNPM` are supported. Please open an issue in the repository if you would like to see any other manager supported.".to_owned()))
    }
}

static NPM_PACKAGE_MANAGER: Lazy<PackageManager> = Lazy::new(|| PackageManager {
    install: "install".into(),
    remove: "uninstall".into(),
    script_executor: "npx".into(),
    command_executor: "npm".into(),
});

static PNPM_PACKAGE_MANAGER: Lazy<PackageManager> = Lazy::new(|| PackageManager {
    install: "install".into(),
    remove: "remove".into(),
    script_executor: "pnpm".into(),
    command_executor: "pnpm".into(),
});

static YARN_PACKAGE_MANAGER: Lazy<PackageManager> = Lazy::new(|| PackageManager {
    install: "add".into(),
    remove: "remove".into(),
    script_executor: "yarn".into(),
    command_executor: "yarn".into(),
});
