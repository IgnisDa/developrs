use super::LibraryError;
use duct::cmd;
use once_cell::sync::Lazy;
use std::{
    env::current_dir,
    fs::read_dir,
    io::{BufRead, BufReader},
};

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
}

impl CommandExecutor {
    fn new(
        install: String,
        remove: String,
        script_executor: String,
        command_executor: String,
    ) -> Self {
        Self {
            install,
            remove,
            script_executor,
            command_executor,
            command_to_execute: vec![],
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

    pub fn execute(self) {
        let command = cmd(self.command_executor, self.command_to_execute);
        info!("Calling dependency installation command");
        let reader = command.stderr_to_stdout().reader().unwrap();
        let lines = BufReader::new(reader).lines();
        for line in lines {
            println!("{}", line.unwrap());
        }
    }
}

#[derive(Clone)]
pub struct PackageManager {
    /// the name of the manager
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
    pub fn get_command_executor() -> Result<CommandExecutor, LibraryError> {
        let dir = read_dir(current_dir().unwrap()).unwrap();
        for file in dir {
            let executor =
                match file.unwrap().file_name().to_os_string().to_str().unwrap() {
                    "yarn.lock" => CommandExecutor::new(
                        YARN_PACKAGE_MANAGER.clone().install,
                        YARN_PACKAGE_MANAGER.clone().remove,
                        YARN_PACKAGE_MANAGER.clone().script_executor,
                        YARN_PACKAGE_MANAGER.clone().command_executor,
                    ),
                    "pnpm-lock.yaml" => CommandExecutor::new(
                        PNPM_PACKAGE_MANAGER.clone().install,
                        PNPM_PACKAGE_MANAGER.clone().remove,
                        PNPM_PACKAGE_MANAGER.clone().script_executor,
                        PNPM_PACKAGE_MANAGER.clone().command_executor,
                    ),
                    "package-lock.json" => CommandExecutor::new(
                        NPM_PACKAGE_MANAGER.clone().install,
                        NPM_PACKAGE_MANAGER.clone().remove,
                        NPM_PACKAGE_MANAGER.clone().script_executor,
                        NPM_PACKAGE_MANAGER.clone().command_executor,
                    ),
                    _ => continue,
                };
            return Ok(executor);
        }
        Err(LibraryError)
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
