use std::{fs::canonicalize, path::PathBuf};

pub const WORKSPACE_FILE: &str = "workspace.json";
pub const WORKSPACE_IDENTIFIER: &str = "workspace";
pub const PACKAGE_JSON_BACKUP_FILE: &str = "package.backup.json";
pub const PACKAGE_JSON_FILE: &str = "package.json";
pub const PROJECT_FILE: &str = "project.json";
pub const DEPENDENCIES_KEY: &str = "dependencies";
pub const DEVELOPMENT_DEPENDENCIES_KEY: &str = "devDependencies";
pub const REQUIRED_KEY: &str = "required";
pub const DEVELOPMENT_KEY: &str = "development";

pub fn workspace_file() -> PathBuf {
    canonicalize(WORKSPACE_FILE).unwrap()
}
