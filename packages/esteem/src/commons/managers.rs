use duct::{cmd, Expression};
use std::{
    borrow::BorrowMut,
    io::{BufRead, BufReader},
};

pub trait GetCommand {
    fn get_command(&mut self) -> &mut Expression;
    fn set_command(&mut self, new_expression: Expression);
}

pub trait AddNpmDependencies: GetCommand {
    fn add_dependencies(&mut self, to_add: Vec<String>) {
        let new_expression = self.get_command().before_spawn(move |cmd| {
            cmd.arg("install");
            to_add.clone().iter().for_each(|f| {
                cmd.arg(f);
            });
            Ok(())
        });
        self.set_command(new_expression);
    }
}

pub trait ExecuteNpmPackageManager: GetCommand {
    fn execute(&mut self) {
        info!("Calling dependency installation command");
        let child = self.get_command();
        let reader = child.stderr_to_stdout().reader().unwrap();
        let lines = BufReader::new(reader).lines();
        for line in lines {
            println!("{}", line.unwrap());
        }
    }
}

pub trait AddNpmDependenciesAndExecuteNpmPackageManager:
    AddNpmDependencies + ExecuteNpmPackageManager
{
}

/// The NPM package manager
pub struct NpmManager {
    command: Expression,
}

impl NpmManager {
    pub fn new() -> Self {
        let args: Vec<&str> = vec![];
        let command = cmd("npm", &args);
        Self { command }
    }
}

impl GetCommand for NpmManager {
    fn get_command(&mut self) -> &mut Expression {
        self.command.borrow_mut()
    }
    fn set_command(&mut self, new_expression: Expression) {
        self.command = new_expression;
    }
}

impl AddNpmDependencies for NpmManager {}

impl ExecuteNpmPackageManager for NpmManager {}

impl AddNpmDependenciesAndExecuteNpmPackageManager for NpmManager {}

/// The PNPM package manager
pub struct PnpmManager {
    command: Expression,
}

impl PnpmManager {
    pub fn new() -> Self {
        let args: Vec<&str> = vec![];
        let command = cmd("pnpm", &args);
        Self { command }
    }
}

impl GetCommand for PnpmManager {
    fn get_command(&mut self) -> &mut Expression {
        self.command.borrow_mut()
    }
    fn set_command(&mut self, new_expression: Expression) {
        self.command = new_expression;
    }
}

impl AddNpmDependencies for PnpmManager {}

impl ExecuteNpmPackageManager for PnpmManager {}

impl AddNpmDependenciesAndExecuteNpmPackageManager for PnpmManager {}

/// The Yarn package manager
pub struct YarnManager {
    command: Expression,
}

impl YarnManager {
    pub fn new() -> Self {
        let args: Vec<&str> = vec![];
        let command = cmd("yarn", &args);
        Self { command }
    }
}

impl GetCommand for YarnManager {
    fn get_command(&mut self) -> &mut Expression {
        self.command.borrow_mut()
    }
    fn set_command(&mut self, new_expression: Expression) {
        self.command = new_expression;
    }
}

impl AddNpmDependencies for YarnManager {
    fn add_dependencies(&mut self, to_add: Vec<String>) {
        let new_expression = self.get_command().before_spawn(move |cmd| {
            cmd.arg("add");
            to_add.clone().iter().for_each(|f| {
                cmd.arg(f);
            });
            Ok(())
        });
        self.set_command(new_expression);
    }
}

impl ExecuteNpmPackageManager for YarnManager {}

impl AddNpmDependenciesAndExecuteNpmPackageManager for YarnManager {}
