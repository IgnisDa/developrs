use core::fmt;
use std::{error::Error, fs::write, path::PathBuf};
mod constants;
mod dependencies;
mod graph;
mod managers;
mod project;
mod utils;
mod workspace;
use serde::Serialize;
pub use {
    cli::{
        perform_add, perform_init, perform_install_isolated, perform_remove,
        perform_workspace_add, perform_workspace_remove, utils_get_dependencies,
    },
    utils::get_all_project_names,
};
mod cli;

#[macro_use]
extern crate log;

#[derive(Debug)]
pub struct LibraryError;

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Library error")
    }
}

impl Error for LibraryError {}

pub trait Command {
    fn execute(&self);
}

/// Used to add a required dependency to a project or workspace
pub trait AddEsteemRequiredDependency {
    fn add_required_dependency(&mut self, dependency: String);
}

/// Used to add a development dependency to a project or workspace
pub trait AddEsteemDevelopmentDependency {
    fn add_development_dependency(&mut self, dependency: String);
}

/// Used to remove a required dependency to a project or workspace
pub trait RemoveEsteemRequiredDependency {
    fn remove_required_dependency(
        &mut self,
        dependency: String,
    ) -> Result<(), LibraryError>;
}

/// Used to remove a development dependency to a project or workspace
pub trait RemoveEsteemDevelopmentDependency {
    fn remove_development_dependency(
        &mut self,
        dependency: String,
    ) -> Result<(), LibraryError>;
}

/// Used to write dependencies to a file
pub trait WriteDependencies
where
    Self: Serialize,
{
    fn get_path(&self) -> PathBuf;

    fn write_dependencies(&self) {
        info!("Writing new dependencies to {:?}", self.get_path());
        let to_write = serde_json::to_string_pretty(self).unwrap();
        write(self.get_path(), to_write).unwrap();
    }
}
