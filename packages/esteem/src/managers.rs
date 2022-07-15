use super::LibraryError;
use duct::{cmd, Expression};
use std::{
    env::current_dir,
    fs::read_dir,
    io::{BufRead, BufReader},
};

enum Manager {
    Npm,
    Yarn,
    Pnpm,
}

pub struct PackageManager {
    /// the name of the package manager
    name: Manager,

    /// the command that will be executed
    command: Expression,
}

impl PackageManager {
    pub fn from_current_directory() -> Result<Self, LibraryError> {
        let dir = read_dir(current_dir().unwrap()).unwrap();
        let args: Vec<&str> = vec![];
        for file in dir {
            match file.unwrap().file_name().to_os_string().to_str().unwrap() {
                "yarn.lock" => {
                    let command = cmd("yarn", &args);
                    return Ok(Self {
                        name: Manager::Yarn,
                        command,
                    });
                }
                "pnpm-lock.yaml" => {
                    let command = cmd("pnpm", &args);
                    return Ok(Self {
                        name: Manager::Pnpm,
                        command,
                    });
                }
                "package-lock.json" => {
                    let command = cmd("npm", &args);
                    return Ok(Self {
                        name: Manager::Npm,
                        command,
                    });
                }
                _ => continue,
            }
        }
        Err(LibraryError)
    }

    pub fn add_dependencies(&mut self, to_add: Vec<String>) {
        let install_command = match self.name {
            Manager::Npm => "install",
            Manager::Pnpm => "install",
            Manager::Yarn => "add",
        };
        self.command = self.command.before_spawn(move |cmd| {
            cmd.arg(install_command);
            to_add.clone().iter().for_each(|f| {
                cmd.arg(f);
            });
            Ok(())
        });
    }

    pub fn remove_dependencies(&mut self, to_remove: Vec<String>) {
        let install_command = match self.name {
            Manager::Npm => "remove",
            Manager::Pnpm => "remove",
            Manager::Yarn => "remove",
        };
        self.command = self.command.before_spawn(move |cmd| {
            cmd.arg(install_command);
            to_remove.clone().iter().for_each(|f| {
                cmd.arg(f);
            });
            Ok(())
        });
    }

    pub fn execute(&mut self) {
        info!("Calling dependency installation command");
        let reader = self.command.stderr_to_stdout().reader().unwrap();
        let lines = BufReader::new(reader).lines();
        for line in lines {
            println!("{}", line.unwrap());
        }
    }
}
