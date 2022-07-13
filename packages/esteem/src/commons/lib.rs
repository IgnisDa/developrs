use core::fmt;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs};

use super::constants::WORKSPACE_FILE;

#[derive(Debug)]
pub struct LibraryError;

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Library error")
    }
}

impl Error for LibraryError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub projects: HashMap<String, String>,
}

impl Workspace {
    pub fn new() -> Result<Self, LibraryError> {
        let workspace_file = fs::read_to_string(WORKSPACE_FILE);
        match workspace_file {
            Ok(data) => Ok(serde_json::from_str(&data).unwrap()),
            Err(_) => {
                trace!("Unable to find file: {:?}", WORKSPACE_FILE);
                Err(LibraryError)
            }
        }
    }
}

pub trait Command {
    fn execute(&self);
}

#[derive(Debug)]
pub enum PackageManager {
    Yarn,
    Pnpm,
    Npm,
}
