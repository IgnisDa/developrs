use std::{fs::canonicalize, path::PathBuf};

pub(crate) const WORKSPACE_FILE: &str = "workspace.json";
pub(crate) const PACKAGE_JSON_BACKUP_FILE: &str = "package.backup.json";
pub(crate) const PACKAGE_JSON_FILE: &str = "package.json";
pub(crate) const APPLICATION_CONFIGURATION_FILE: &str = "esteem.toml";
pub(crate) const PROJECT_FILE: &str = "project.json";
pub(crate) const REQUIRED_KEY: &str = "required";
pub(crate) const DEVELOPMENT_KEY: &str = "development";

pub fn workspace_file() -> PathBuf {
    canonicalize(WORKSPACE_FILE).unwrap()
}
