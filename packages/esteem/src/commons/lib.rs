use core::fmt;
use std::error::Error;

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

#[derive(Debug)]
pub enum PackageManager {
    Yarn,
    Pnpm,
    Npm,
}

/// Used to add a required dependency to a project or workspace
pub trait AddEsteemRequiredDependency {
    fn add_required_dependency(&mut self, dependency: String);
}

/// Used to add a development dependency to a project or workspace
pub trait AddEsteemDevelopmentDependency {
    fn add_development_dependency(&mut self, dependency: String);
}

/// Used to write dependencies to a file
pub trait WriteDependencies {
    fn write_dependencies(&self);
}
